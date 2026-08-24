class SqueakyClean {
    // 1. Replace spaces with underscores
    // 2. Convert kebab case to normal case { "a-bc" => "aBc" }
    // 3. Convert leetspeak to normal text { 4, 3, 0, 1, 7 => a, e, o, l, t }
    // 4. Remove all characters that are not letters { "a$#.b" => "ab" }
    //
    // It needs remove non-letters before replacing spaces
    static String clean(String identifier) {
        if (identifier.isEmpty())
            return "";

        StringBuilder sb = new StringBuilder();

        boolean capitalize_next = false;
        for (char character : identifier.toCharArray()) {
            if (character == '-') {
                capitalize_next = true;
            } else if (Character.isLetter(character)) {
                if (capitalize_next) {
                    sb.append(Character.toUpperCase(character));
                    capitalize_next = false;
                } else {
                    sb.append(character);
                }
            } else if (Character.isDigit(character)) {
                switch (character) {
                    case '4':
                        sb.append("a");
                        break;
                    case '3':
                        sb.append("e");
                        break;
                    case '0':
                        sb.append("o");
                        break;
                    case '1':
                        sb.append("l");
                        break;
                    case '7':
                        sb.append("t");
                        break;
                }
            } else if (Character.isWhitespace(character)) {
                sb.append('_');
            } else {
                continue;
            }
        }
        return sb.toString();
    }
}
