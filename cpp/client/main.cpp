#include <winrt/server.h>
#include <assert.h>

using namespace winrt::Server;

int main()
{
    Reader reader;
    assert(reader.P0() == L"P0");
    assert(reader.P1() == L"P1");

    Writer writer;
    writer.P0(L"0");
    writer.P1(L"1");

    assert(writer.P0() == L"0");
    assert(writer.P1() == L"1");
}