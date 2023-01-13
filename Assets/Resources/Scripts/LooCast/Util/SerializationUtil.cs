using System.IO;
using UnityEngine;


namespace LooCast.Util
{
    using LooCast.Data;
    
    public static class SerializationUtil
    {
        public static void SaveData<T>(T data, string path)
        {
            path = Path.Combine(Data.Path, path);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using (FileStream fileStream = File.Open(path, FileMode.Create))
            {
                var binaryFormatter = new System.Runtime.Serialization.Formatters.Binary.BinaryFormatter();
                binaryFormatter.Serialize(fileStream, data);
            }
        }

        public static T LoadData<T>(string path)
        {
            path = Path.Combine(Data.Path, path);
            if (File.Exists(path))
            {
                using (FileStream fileStream = File.Open(path, FileMode.Open))
                {
                    var binaryFormatter = new System.Runtime.Serialization.Formatters.Binary.BinaryFormatter();
                    return (T)binaryFormatter.Deserialize(fileStream);
                }
            }
            else
            {
                throw new FileNotFoundException($"Data at {path} could not be found!");
            }
        }
    }
}
