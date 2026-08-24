public class CarsAssemble {

  // Speed ranges from 0 (off) to 10
  // Production rate scales linearly with speed
  // At speed 1, 221 cars are produced per hour
  //
  // Higher speeds increase the likelihood that faulty cars are produced:
  //
  // - `1` to `4`: 100% success rate.
  // - `5` to `8`: 90% success rate.
  // - `9`: 80% success rate.
  // - `10`: 77% success rate.

  private final int HOURLY_PRODUCTION_RATE = 221;

  public double productionRatePerHour(int speed) {
    double success_rate = getSuccessRate(speed);
    return success_rate * HOURLY_PRODUCTION_RATE * speed;
  }

  public int workingItemsPerMinute(int speed) {
    return (int) productionRatePerHour(speed) / 60;
  }

  private double getSuccessRate(int speed) {
    if (speed <= 4) {
      return 1;
    } else if (speed <= 8) { // This will include speed == 5
      return 0.9;
    } else if (speed == 9) {
      return 0.8;
    } else {
      // In this case speed is 10
      return 0.77;
    }
  }
}
