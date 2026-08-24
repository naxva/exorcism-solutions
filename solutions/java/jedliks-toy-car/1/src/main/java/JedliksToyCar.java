import java.text.MessageFormat;

public class JedliksToyCar {

    private int distance = 0;
    private int battery = 100;

    public static JedliksToyCar buy() {
        return new JedliksToyCar();
    }

    public String distanceDisplay() {
        return String.format("Driven %s meters", this.distance);
    }

    public String batteryDisplay() {
        if (this.battery == 0) {
            return "Battery empty";
        }
        return MessageFormat.format("Battery at {0}%", this.battery);
    }

    public void drive() {
        if (this.battery == 0) {
            return;
        }
        this.distance += 20;
        this.battery -= 1;
    }
}
