defmodule SecretHandshake do
  @actions [
    {0b1000, "jump"},
    {0b100, "close your eyes"},
    {0b10, "double blink"},
    {0b1, "wink"},
    {0b10000, "reverse"}
  ]

  @doc """
  Determine the actions of a secret handshake based on the binary
  representation of the given `code`.

  If the following bits are set, include the corresponding action in your list
  of commands, in order from lowest to highest.

  1 = wink
  10 = double blink
  100 = close your eyes
  1000 = jump

  10000 = Reverse the order of the operations in the secret handshake
  """
  @spec commands(code :: integer) :: list(String.t())
  def commands(code), do: reply(code, @actions, [])

  defp reply(_, [], replies), do: replies

  defp reply(code, [{reply_code, reply_string} | actions], replies)
       when Bitwise.band(code, reply_code) != 0 do
    case reply_string do
      "reverse" -> Enum.reverse(replies)
      _ -> reply(code, actions, [reply_string | replies])
    end
  end

  defp reply(code, [_ | actions], replies), do: reply(code, actions, replies)
end
