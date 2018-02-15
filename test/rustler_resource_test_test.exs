defmodule RustlerResourceTestTest do
  use ExUnit.Case
  doctest RustlerResourceTest

  test "greets the world" do
    assert RustlerResourceTest.hello() == :world
  end
end
