defmodule Poker do
  @doc """
  Given a list of poker hands, return a list containing the highest scoring hand.

  If two or more hands tie, return the list of tied hands in the order they were received.

  The basic rules and hand rankings for Poker can be found at:

  https://en.wikipedia.org/wiki/List_of_poker_hands

  For this exercise, we'll consider the game to be using no Jokers,
  so five-of-a-kind hands will not be tested. We will also consider
  the game to be using multiple decks, so it is possible for multiple
  players to have identical cards.

  Aces can be used in low (A 2 3 4 5) or high (10 J Q K A) straights, but do not count as
  a high card in the former case.

  For example, (A 2 3 4 5) will lose to (2 3 4 5 6).

  You can also assume all inputs will be valid, and do not need to perform error checking
  when parsing card values. All hands will be a list of 5 strings, containing a number
  (or letter) for the rank, followed by the suit.

  Ranks (lowest to highest): 2 3 4 5 6 7 8 9 10 J Q K A
  Suits (order doesn't matter): C D H S

  Example hand: ~w(4S 5H 4C 5D 4H) # Full house, 5s over 4s
  """

  @ranks %{
    "2" => 2,
    "3" => 3,
    "4" => 4,
    "5" => 5,
    "6" => 6,
    "7" => 7,
    "8" => 8,
    "9" => 9,
    "10" => 10,
    "J" => 11,
    "Q" => 12,
    "K" => 13,
    "A" => 14
  }

  @variants ~w(
    straight_flush
    4_of_a_kind
    full_house
    flush
    straight
    3_of_a_kind
    2_pair
    pair
    high_card
  )

  @spec best_hand(list(list(String.t()))) :: list(list(String.t()))
  def best_hand(hands) do
    hands = Enum.map(hands, &(&1 |> sort_by_rank_and_frequency))

    Enum.find_value(@variants, fn variant ->
      hands = check_variant(hands, variant)

      if Enum.any?(hands) do
        max_score = max_score(hands)

        hands
        |> Enum.filter(fn hand -> Enum.map(hand, &elem(&1, 0)) == max_score end)
        |> format()
      end
    end)
  end

  defp sort_by_rank_and_frequency(hand) do
    hand =
      hand
      |> Enum.map(fn hand ->
        {rank, suit} = String.split_at(hand, -1)

        {@ranks[rank], suit}
      end)

    Enum.sort_by(
      hand,
      fn {score, _} ->
        count = Enum.count(hand, fn {r, _} -> r == score end)
        {count, score}
      end,
      :desc
    )
  end

  defp check_variant(hands, "straight_flush") do
    Enum.filter(hands, fn
      [{_, s}, {_, s}, {_, s}, {_, s}, {_, s}] = hand ->
        [{c, _}, {l, _}, _, _, _] = hand

        straight = [c, c - 1, c - 2, c - 3, c - 4]
        low_ace_straight = [14, l, l - 1, l - 2, l - 3]

        Enum.map(hand, &elem(&1, 0)) in [straight, low_ace_straight]

      _ ->
        false
    end)
    |> Enum.map(fn
      [{14, s} | rest] -> [{0, s} | rest]
      hand -> hand
    end)
  end

  defp check_variant(hands, "4_of_a_kind") do
    Enum.filter(hands, fn hand ->
      Enum.any?(hand, fn {rank, _suit} ->
        Enum.count(hand, fn {r, _s} -> r == rank end) == 4
      end)
    end)
  end

  defp check_variant(hands, "full_house") do
    Enum.filter(hands, fn
      [{a, _}, {a, _}, {a, _}, {b, _}, {b, _}] -> true
      _ -> false
    end)
  end

  defp check_variant(hands, "flush") do
    Enum.filter(hands, fn
      [{_, s}, {_, s}, {_, s}, {_, s}, {_, s}] -> true
      _ -> false
    end)
  end

  defp check_variant(hands, "straight") do
    Enum.filter(hands, fn hand ->
      [{c, _}, {l, _}, _, _, _] = hand
      # ace can start a straight
      straight = [c, c - 1, c - 2, c - 3, c - 4]
      low_ace_straight = [14, l, l - 1, l - 2, l - 3]

      # case hand do
      #
      # end

      Enum.map(hand, &elem(&1, 0)) in [straight, low_ace_straight]
    end)
    |> Enum.map(fn
      [{14, s} | rest] -> [{0, s} | rest]
      hand -> hand
    end)
  end

  defp check_variant(hands, "3_of_a_kind") do
    Enum.filter(hands, fn hand ->
      Enum.any?(hand, fn {rank, _suit} ->
        Enum.count(hand, fn {r, _s} -> r == rank end) == 3
      end)
    end)
  end

  defp check_variant(hands, "2_pair") do
    Enum.filter(hands, fn hand ->
      Enum.count(hand, fn {rank, _suit} ->
        Enum.count(hand, fn {r, _s} -> r == rank end) == 2
      end) == 4
    end)
  end

  defp check_variant(hands, "pair") do
    Enum.filter(hands, fn hand ->
      Enum.any?(hand, fn {rank, _suit} ->
        Enum.count(hand, fn {r, _s} -> r == rank end) == 2
      end)
    end)
  end

  defp check_variant(hands, "high_card") do
    hands
  end

  defp max_score(hands) do
    hands
    |> Enum.max_by(fn hand -> Enum.map(hand, fn {score, _} -> score end) end, fn -> [] end)
    |> Enum.map(&elem(&1, 0))
  end

  defp format(hands) do
    Enum.map(hands, fn hand ->
      Enum.map(hand, fn {rank, suit} ->
        rank =
          case rank do
            11 -> "J"
            12 -> "Q"
            13 -> "K"
            14 -> "A"
            0 -> "A"
            num -> num
          end

        "#{rank}#{suit}"
      end)
    end)
  end
end
