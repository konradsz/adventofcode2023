defmodule Aoc do
  def get_digit(line) do
    {first, tail} = String.next_grapheme(line)

    case Integer.parse(first) do
      {digit, _} -> digit
      :error -> get_digit(tail)
    end
  end
end

{:ok, input} = File.read("input")

lines =
  input |> String.split("\n", trim: true)

result =
  lines
  |> Enum.map(fn line ->
    Aoc.get_digit(line) * 10 + Aoc.get_digit(String.reverse(line))
  end)
  |> Enum.sum()

IO.puts(result)
