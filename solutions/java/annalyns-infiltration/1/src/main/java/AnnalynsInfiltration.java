class AnnalynsInfiltration {
  // A fast attack can be made if the knight is sleeping
  public static boolean canFastAttack(boolean knightIsAwake) {
    return !knightIsAwake;
  }

  // A group can be spied if at least one member is awake
  public static boolean canSpy(
      boolean knightIsAwake, boolean archerIsAwake, boolean prisonerIsAwake) {
    return knightIsAwake || archerIsAwake || prisonerIsAwake;
  }

  // The prisoner can be signaled if they are awake and the archer is asleep
  public static boolean canSignalPrisoner(boolean archerIsAwake, boolean prisonerIsAwake) {
    return !archerIsAwake && prisonerIsAwake;
  }

  // The prisoner can be freed if either:
  // 1. Anna has her pet dog and the archer is asleep, as the knight is afraid of the dog
  // 2. If the prisoner is awake and both the knight and archer are asleep
  public static boolean canFreePrisoner(
      boolean knightIsAwake,
      boolean archerIsAwake,
      boolean prisonerIsAwake,
      boolean petDogIsPresent) {
    // !(knightIsAwake || archerIsAwake) == !knightIsAwake && !archerIsAwake, by De Morgan's Law
    return (petDogIsPresent && !archerIsAwake)
        || (prisonerIsAwake && !(knightIsAwake || archerIsAwake));
  }
}
