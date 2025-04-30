#include <fstream>
#include <iostream>
#include <iomanip>
#include <string>
#include <cstdint>
#include <filesystem>
#include "CRCpp/inc/CRC.h"

int main(int argc, char* argv[])
{
    std::string binFilePath;
    std::ifstream binFile;
    
    /* Bail if enough arguments are not provided */
    if (argc < 2)
    {
        std::cout << "An input file with .bin extension must be provided. Exiting ..." << std::endl;
        return 1;
    }
     
    /* Get the path of the .bin file */
    binFilePath = std::string(argv[1]);
    
    // Bail if file name doesn't have .bin extension
    if (binFilePath.length() < 4 || binFilePath.substr(binFilePath.length() - 4) != ".bin")
    {
        std::cout << "The input file must have .bin extension. Exiting ..." << std::endl;
        return 1;
    }
    
    // Open the file to check if it exists
    binFile.open(binFilePath.c_str(), std::ios::binary);
    if (!binFile.is_open())
    {
        std::cout << "Could not locate file: " << binFilePath << ". Exiting ..." << std::endl;
        return 1;
    }
    
    /* Get file size */
    binFile.seekg(0, std::ios::end);
    size_t binFileSize = binFile.tellg();
    binFile.seekg(0, std::ios::beg);
    
    /* Bail if it is > 252 bytes in size */
    if (binFileSize > 252)
    {
        std::cout << "The input must be 252 Bytes in size at max. Exiting ..." << std::endl;
        binFile.close();
        return 1;
    }
    
    /* Load the file contents into an array */
    unsigned char binFileData[252] = {0};
    binFile.read(reinterpret_cast<char*>(binFileData), 252);
    binFile.close();
    
    /* Calculate CRC32 for the 252 bytes of data */
    unsigned char crc[4] = {0};
    *reinterpret_cast<uint32_t*>(crc) = CRC::Calculate(binFileData, 252, CRC::CRC_32_MPEG2());
    
    /* Get the directory path of the input file */
    std::string dirPath = binFilePath.substr(0, binFilePath.find_last_of("/\\"));
    
    /* Create the output file path */
    std::string outFilePath = dirPath + "/crc.c";
    
    /* Output the contents of the crc array to crc.c */
    std::ofstream cppFile(outFilePath.c_str());
    if (!cppFile.is_open())
    {
        std::cout << "Failed to create output file: " << outFilePath << std::endl;
        return 1;
    }
    
    cppFile << "__attribute__((section(\".crc\"))) unsigned char crc[4] = {";
    
    /* Write each byte of CRC with proper formatting */ 
    for (int i = 0; i < 3; i++) {
        cppFile << "0x" << std::hex << std::setw(2) << std::setfill('0')
                << static_cast<unsigned int>(crc[i]) << ", ";
    }
    cppFile << "0x" << std::hex << std::setw(2) << std::setfill('0')
            << static_cast<unsigned int>(crc[3]) << "};";
    
    cppFile.close();
    
    std::cout << "Generated CRC file: " << outFilePath << std::endl;
    return 0;
}