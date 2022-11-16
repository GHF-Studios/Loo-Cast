using System;
using UnityEngine;

namespace LooCast.Game
{
    public struct RuntimeData
    {
        public string JsonData { get; private set; }
        public string Identifier { get; private set; }
        public string DataPath { get; private set; }
        public string ObjectPrefabPath { get; private set; }

        public RuntimeData(string jsonData, string identifier, string dataPath, string objectPrefabPath)
        {
            JsonData = jsonData;
            Identifier = identifier;
            DataPath = dataPath;
            ObjectPrefabPath = objectPrefabPath;
        }
    }
}
