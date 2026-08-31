public class LogLine {
    private String logLine;

    public LogLine(String logLine) {
        this.logLine = logLine;
    }

    public LogLevel getLogLevel() {
        String logLevel = this.logLine.substring(1, 4);

        return switch (logLevel) {
            case "INF" -> LogLevel.INFO;
            case "TRC" -> LogLevel.TRACE;
            case "DBG" -> LogLevel.DEBUG;
            case "WRN" -> LogLevel.WARNING;
            case "ERR" -> LogLevel.ERROR;
            case "FTL" -> LogLevel.FATAL;
            default -> LogLevel.UNKNOWN;
        };
    }

    public String getOutputForShortLog() {
        LogLevel level = this.getLogLevel();
        int logLevel = level.getCode();

        return logLevel + ":" + this.logLine.substring(7);
    }
}
