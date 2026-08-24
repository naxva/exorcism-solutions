class BirdWatcher {
  private final int[] birdsPerDay;

  public BirdWatcher(int[] birdsPerDay) {
    this.birdsPerDay = birdsPerDay.clone();
  }

  public static int[] getLastWeek() {
    int[] birds_last_week = {0, 2, 5, 3, 7, 8, 4};
    return birds_last_week;
  }

  public int getToday() {
    return birdsPerDay[birdsPerDay.length - 1];
  }

  public void incrementTodaysCount() {
    int index_today = birdsPerDay.length - 1;
    birdsPerDay[index_today]++;
  }

  public boolean hasDayWithoutBirds() {
    for (int bird_count : birdsPerDay) {
      if (bird_count == 0) {
        return true;
      }
    }
    return false;
  }

  public int getCountForFirstDays(int numberOfDays) {
    numberOfDays = Math.min(numberOfDays, birdsPerDay.length);
    int total_birds = 0;
    for (int i = 0; i < numberOfDays; i++) {
      total_birds += birdsPerDay[i];
    }

    return total_birds;
  }

  public int getBusyDays() {
    int busy_days = 0;
    for (int bird_count : birdsPerDay) {
      if (bird_count >= 5) {
        busy_days += 1;
      }
    }

    return busy_days;
  }
}
