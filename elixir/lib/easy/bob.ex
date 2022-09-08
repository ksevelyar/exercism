defmodule Bob do
  @spec hey(String.t()) :: String.t()
  def hey(input) do
    trimmed_input = String.trim(input)
    reply(yell?(trimmed_input), question?(trimmed_input), empty?(trimmed_input))
  end

  defp yell?(input), do: String.match?(input, ~r/[[:alpha:]]/) && input == String.upcase(input)
  defp question?(input), do: String.ends_with?(input, "?")
  defp empty?(input), do: input == ""

  defp reply(_is_yell, _is_question, true), do: "Fine. Be that way!"
  defp reply(true, true, _is_empty), do: "Calm down, I know what I'm doing!"
  defp reply(true, _is_question, _is_empty), do: "Whoa, chill out!"
  defp reply(_is_yell, true, _is_empty), do: "Sure."
  defp reply(_is_yell, _is_question, _is_empty), do: "Whatever."
end
