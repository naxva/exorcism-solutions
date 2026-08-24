import java.util.Objects;

class CalculatorConundrum {

    public String calculate(int operand1, int operand2, String operation) {
        if (Objects.isNull(operation)) {
            throw new IllegalArgumentException("Operation cannot be null");
        }
        if (operation.isEmpty()) {
            throw new IllegalArgumentException("Operation cannot be empty");
        }
        switch (operation) {
            case "+":
                return operand1 + " + " + operand2 + " = " + (operand1 + operand2);
            case "*":
                return operand1 + " * " + operand2 + " = " + (operand1 * operand2);
            case "/":
                try {
                    return operand1 + " / " + operand2 + " = " + (operand1 / operand2);
                } catch (Exception e) {
                    throw new IllegalOperationException("Division by zero is not allowed", e);
                }
            default:
                throw new IllegalOperationException("Operation '" + operation + "' does not exist");
        }
    }
}
