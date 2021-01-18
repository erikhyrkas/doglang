package org.dog;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;

/**
 * This is a quick and dirty utility to format the parser rules for dog.
 */
public class ParserGenerator {


    public static void main(String[] args) {
        List<String> text = readFile("src/main/resources/parser_def.txt");
        List<ParserLine> parserLines = new ArrayList<>();
        for (String line : text) {
            if (!line.trim().isEmpty()) {
                ParserLine parserLine = new ParserLine();
                int commentStart = line.indexOf("//");
                String remainder = line;
                if (commentStart >= 0) {
                    parserLine.comment = line.substring(commentStart);
                    remainder = line.substring(0, commentStart);
                }
                String[] parts = remainder.split("[:]");
                if (parts.length == 2) {
                    parserLine.label = parts[0].trim();
                    parserLine.rule = parseRule(parts[1].trim());
                }
                parserLine.original = line;
                parserLines.add(parserLine);
            }
        }

        for (ParserLine parserLine : parserLines) {
            if (parserLine.label != null) {
                System.out.println("\t// " + parserLine.original);
                //System.out.println("\t// "+parserLine.label + ": " + parserLine.rule);
                if (parserLine.rule.ruleType == RuleType.token) {
                    if (parserLine.rule.frequency != Frequency.one) {
                        throw new RuntimeException("Unexpected Frequency: " + parserLine.rule.frequency);
                    }
                    System.out.printf("\tresult.insert(\"%s\", create_label_match(vec![%s]));", parserLine.label, labelsToString(parserLine.rule.labels));
                } else if (parserLine.rule.ruleType == RuleType.and) {
                    switch (parserLine.rule.frequency) {
                        case one -> System.out.printf("\tresult.insert(\"%s\", create_and_rule_once( vec![%s]));", parserLine.label, labelsToString(parserLine.rule.labels));
                        case one_or_more -> System.out.printf("\tresult.insert(\"%s\", create_and_rule(RuleRepeats::OneOrMore, vec![%s]));", parserLine.label, labelsToString(parserLine.rule.labels));
                        case zero_or_one -> System.out.printf("\tresult.insert(\"%s\", create_and_rule(RuleRepeats::ZeroOrOne, vec![%s]));", parserLine.label, labelsToString(parserLine.rule.labels));
                        case zero_or_more -> System.out.printf("\tresult.insert(\"%s\", create_and_rule(RuleRepeats::ZeroOrMore, vec![%s]));", parserLine.label, labelsToString(parserLine.rule.labels));
                    }
                } else {
                    switch (parserLine.rule.frequency) {
                        case one -> System.out.printf("\tresult.insert(\"%s\", create_or_rule_once( vec![%s]));", parserLine.label, labelsToString(parserLine.rule.labels));
                        case one_or_more -> System.out.printf("\tresult.insert(\"%s\", create_or_rule(RuleRepeats::OneOrMore, vec![%s]));", parserLine.label, labelsToString(parserLine.rule.labels));
                        case zero_or_one -> System.out.printf("\tresult.insert(\"%s\", create_or_rule(RuleRepeats::ZeroOrOne, vec![%s]));", parserLine.label, labelsToString(parserLine.rule.labels));
                        case zero_or_more -> System.out.printf("\tresult.insert(\"%s\", create_or_rule(RuleRepeats::ZeroOrMore, vec![%s]));", parserLine.label, labelsToString(parserLine.rule.labels));
                    }
                }
            }
            if (parserLine.comment != null) {
                System.out.print(parserLine.comment);
            }
            System.out.println();
        }
    }

    private static String labelsToString(List<String> labels) {
        StringBuilder builder = new StringBuilder();
        String delim = "";
        for (String label : labels) {
            builder.append(delim).append('"').append(label).append('"');
            delim = ", ";
        }
        return builder.toString();
    }

    private static ParserRule parseRule(String rule) {
        ParserRule result = new ParserRule();
        if (rule.contains("*")) {
            result.frequency = Frequency.zero_or_more;
        } else if (rule.contains("?")) {
            result.frequency = Frequency.zero_or_one;
        } else if (rule.contains("+")) {
            result.frequency = Frequency.one_or_more;
        } else {
            result.frequency = Frequency.one;
        }
        String cleanRule = rule.replaceAll("[()*?+ ]", "");
        if (cleanRule.contains("||")) {
            result.ruleType = RuleType.or;
            result.labels = Arrays.asList(cleanRule.split("[|][|]"));
        } else if (cleanRule.contains("&&")) {
            if (cleanRule.startsWith("_")) {
                result.ruleType = RuleType.token;
            } else {
                result.ruleType = RuleType.and;
            }
            result.labels = Arrays.asList(cleanRule.split("[&][&]"));
        } else {
            if (cleanRule.startsWith("_")) {
                result.ruleType = RuleType.token;
            } else {
                result.ruleType = RuleType.and;
            }
            result.labels = Collections.singletonList(cleanRule);
        }
        return result;
    }

    private static List<String> readFile(String parserFile) {
        List<String> result = new ArrayList<>();
        File file = new File(parserFile);
        try (BufferedReader reader = new BufferedReader(new FileReader(file))) {
            String line = reader.readLine();
            while (line != null) {
                result.add(line);
                line = reader.readLine();
            }
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
        return result;
    }

    private static class ParserLine {
        public String comment;
        public String label;
        public ParserRule rule;
        public String original;

        @Override
        public String toString() {
            return "ParserLine{" +
                    "comment='" + comment + '\'' +
                    ", label='" + label + '\'' +
                    ", rule=" + rule +
                    '}';
        }
    }

    private static class ParserRule {
        public Frequency frequency;
        public RuleType ruleType;
        public List<String> labels;

        @Override
        public String toString() {
            return "ParserRule{" +
                    "frequency=" + frequency +
                    ", ruleType=" + ruleType +
                    ", labels=" + labels +
                    '}';
        }
    }

    private enum Frequency {
        one,
        one_or_more,
        zero_or_one,
        zero_or_more
    }

    private enum RuleType {
        and,
        or,
        token
    }
}
