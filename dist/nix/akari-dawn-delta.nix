# Akari Dawn — delta options for Nix
# Generated from palette/akari-dawn.toml
{
  light = true;
  syntax-theme = "Akari Dawn";
  line-numbers = true;

  # Diff styling
  minus-style = ''syntax "#D2C3BC"'';
  minus-emph-style = ''syntax "#D2C3BC" bold'';
  plus-style = ''syntax "#CBCABD"'';
  plus-emph-style = ''syntax "#CBCABD" bold'';

  # Line numbers
  line-numbers-minus-style = "#6A2828";
  line-numbers-plus-style = "#314B29";
  line-numbers-zero-style = "#514B45";
  line-numbers-left-style = "#CABEAE";
  line-numbers-right-style = "#CABEAE";

  # Hunk header
  hunk-header-style = "file line-number #304050";
  hunk-header-decoration-style = "#CABEAE ul";
  hunk-header-file-style = "#8A4530";
  hunk-header-line-number-style = "#304050";

  # File header
  file-style = ''"#8A4530" bold'';
  file-decoration-style = "#CABEAE ul";

  # Commit decoration
  commit-decoration-style = "#8A4530 box";

  # Blame
  blame-palette = "#E4DED6 #CFC4B6 #DDD2C9 #D9D1C6";

  # Merge conflict
  merge-conflict-begin-symbol = "▼";
  merge-conflict-end-symbol = "▲";
  merge-conflict-ours-diff-header-style = ''"#B07840" bold'';
  merge-conflict-theirs-diff-header-style = ''"#304050" bold'';
}
