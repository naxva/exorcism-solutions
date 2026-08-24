class NeedForSpeed {
    private int speed;
    private int batteryDrain;
    private int batteryPercentage = 100;
    private int distance = 0;

    NeedForSpeed(int speed, int batteryDrain) {
        this.speed = speed;
        this.batteryDrain = batteryDrain;
    }

    public boolean batteryDrained() {
        return this.batteryPercentage < this.batteryDrain;
    }

    public int distanceDriven() {
        return this.distance;
    }

    public void drive() {
        if (this.batteryDrained()) {
            return;
        }
        this.distance += this.speed;
        this.batteryPercentage -= this.batteryDrain;
    }

    public static NeedForSpeed nitro() {
        return new NeedForSpeed(50, 4);
    }

    public int maxDistance() {
        int batteryCycles = 100 / this.batteryDrain;
        return batteryCycles * this.speed;
    }
}

class RaceTrack {
    private int distance;

    RaceTrack(int distance) {
        this.distance = distance;
    }

    public boolean canFinishRace(NeedForSpeed car) {
        return car.maxDistance() >= this.distance;
    }
}
