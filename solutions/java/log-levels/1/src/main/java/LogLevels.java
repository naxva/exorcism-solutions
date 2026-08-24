public class LogLevels {
    // Logs are of form: "[<LEVEL>]: <MESSAGE>"

    public static String message(String logLine) {
        return logLine.split(": ")[1].strip();
    }

    public static String logLevel(String logLine) {
        return logLine.substring(1, logLine.indexOf("]")).toLowerCase();
    }

    public static String reformat(String logLine) {
        String level = logLevel(logLine);
        String message = message(logLine);
        return String.format("%s (%s)", message, level);
    }
}
