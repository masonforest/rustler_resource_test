defmodule NativeModule do
  use Rustler, otp_app: :rustler_resource_test, crate: :native_module

  # When loading a NIF module, dummy clauses for all NIF function are required.
  # NIF dummies usually just error out when called when the NIF is not loaded, as that should never normally happen.
  def get_resource(), do: exit(:nif_not_loaded)
end
