public class Lasagna {
  // TODO: define the 'expectedMinutesInOven()' method
  public static int expectedMinutesInOven() {
    return 40;
  }

  // TODO: define the 'remainingMinutesInOven()' method

  public static int remainingMinutesInOven(int time_elapsed) {
    return expectedMinutesInOven() - time_elapsed;
  }

  // TODO: define the 'preparationTimeInMinutes()' method
  public static int preparationTimeInMinutes(int layers) {
    return 2 * layers;
  }

  // TODO: define the 'totalTimeInMinutes()' method
  public int totalTimeInMinutes(int layers, int time_elapsed) {
    return preparationTimeInMinutes(layers) + time_elapsed;
  }
}
