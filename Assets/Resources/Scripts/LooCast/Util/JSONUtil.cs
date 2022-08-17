using System.IO;
using UnityEngine;

namespace LooCast.Util
{
    public static class JSONUtil
    {
        private static string dataPath = $"{Application.dataPath}/Data/";

        public static void SaveData<T>(T data, string path)
        {
            path = dataPath + path;
            string json = JsonUtility.ToJson(data, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter writer = new StreamWriter(path);
            writer.Write(json);
            Debug.Log($"Saved data at {path}");
        }

        public static T LoadData<T>(string path)
        {
            string defaultPath = dataPath + "Default/" + path;
            path = dataPath + path;
            if (File.Exists(path))
            {
                using StreamReader reader = new StreamReader(path);
                string json = reader.ReadToEnd();
                Debug.Log($"Loaded data from {path}");
                return JsonUtility.FromJson<T>(json);
            }
            else
            {
                if (File.Exists(defaultPath))
                {
                    using StreamReader reader = new StreamReader(defaultPath);
                    string json = reader.ReadToEnd();
                    Debug.Log($"Loaded data from {defaultPath}");
                    return JsonUtility.FromJson<T>(json);
                }
                else
                {
                    throw new FileNotFoundException($"Default data at {defaultPath} could not be found!");
                }
            }
        }
    }
}
