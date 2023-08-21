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
            string json = JsonUtility.ToJson(data, true);
            File.WriteAllText(path, json);
        }

        public static T LoadData<T>(string path)
        {
            path = Path.Combine(Data.Path, path);
            if (File.Exists(path))
            {
                string json = File.ReadAllText(path);
                return JsonUtility.FromJson<T>(json);
            }
            else
            {
                throw new FileNotFoundException($"Data at {path} could not be found!");
            }
        }
    }
}
