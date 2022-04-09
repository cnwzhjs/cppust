#include <cppust/cppust.hpp>
#include "nmea/decoder_status.hpp"

nmea::DecoderStatus decode_data(const cppust::u8* buffer, cppust::usize len) {
    return nmea::DecoderStatus::ChecksumError(123, 321);
}

int main() {
    std::cout<<cppust::as_debug(decode_data(nullptr, 0))<<"\n";

    return 0;
}
