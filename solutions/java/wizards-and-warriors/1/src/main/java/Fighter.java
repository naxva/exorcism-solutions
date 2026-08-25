class Fighter {

    boolean isVulnerable() {
        return true;
    }

    int getDamagePoints(Fighter fighter) {
        return 1;
    }
}

class Warrior extends Fighter {

    @Override
    boolean isVulnerable() {
        return false;
    }

    @Override
    int getDamagePoints(Fighter target) {
        return target.isVulnerable() ? 10 : 6;
    }

    @Override
    public String toString() {
        return "Fighter is a Warrior";
    }
}

class Wizard extends Fighter {

    private boolean hasSpellPrepared = false;

    @Override
    public String toString() {
        return "Fighter is a Wizard";
    }

    public void prepareSpell() {
        this.hasSpellPrepared = true;
    }

    @Override
    public boolean isVulnerable() {
        return !this.hasSpellPrepared;
    }

    @Override
    public int getDamagePoints(Fighter target) {
        return this.hasSpellPrepared ? 12 : 3;
    }
}
