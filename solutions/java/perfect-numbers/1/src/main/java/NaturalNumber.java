class NaturalNumber {

    private final int number;
    private final int aliquot_sum;

    NaturalNumber(int number) {
        if (number <= 0) {
            throw new IllegalArgumentException("You must supply a natural number (positive integer)");
        }
        this.number = number;
        this.aliquot_sum = this.getAliquotSum();
    }

    Classification getClassification() {
        if (this.number == this.aliquot_sum) {
            return Classification.PERFECT;
        } else if (this.number < this.aliquot_sum) {
            return Classification.ABUNDANT;
        } else {
            return Classification.DEFICIENT;
        }
    }

    private int getAliquotSum() {
        int total = 0;
        for (int i = 1; i < this.number; i++) {
            if (number % i == 0) {
                total += i;
            }
        }
        return total;
    }
}
