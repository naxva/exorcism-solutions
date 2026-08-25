class ProductionRemoteControlCar
        implements RemoteControlCar, Comparable<ProductionRemoteControlCar> {
    private final int SPEED = 10;
    private int distance = 0;
    private int numberOfVictories = 0;

    public void drive() {
        this.distance += SPEED;
    }

    public int getDistanceTravelled() {
        return this.distance;
    }

    public int getNumberOfVictories() {
        return this.numberOfVictories;
    }

    public void setNumberOfVictories(int numberOfVictories) {
        this.numberOfVictories = numberOfVictories;
    }

    @Override
    public int compareTo(ProductionRemoteControlCar other) {
        return Integer.compare(this.numberOfVictories, other.numberOfVictories);
    }
}
