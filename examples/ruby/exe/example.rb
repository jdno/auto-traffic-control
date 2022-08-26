# frozen_string_literal: true

require "atc/v1/atc_services_pb"
require "grpc"

def main
  service = Atc::V1::AtcService::Stub.new("localhost:4747", :this_channel_is_insecure)

  version = service.get_version(Atc::V1::GetVersionRequest.new).version

  p "Auto Traffic Control is running version #{version.major}.#{version.minor}.#{version.patch}"
end

main
