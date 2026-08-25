class Badge {
    public String print(Integer id, String name, String department) {
        String idString;
        String departmentString;
        if (id != null) {
            idString = String.format("[%d] - ", id);
        } else {
            idString = "";
        }
        if (department == null) {
            departmentString = " - OWNER";
        } else {
            departmentString = " - " + department.toUpperCase();
        }

        return idString + name + departmentString;
    }
}
