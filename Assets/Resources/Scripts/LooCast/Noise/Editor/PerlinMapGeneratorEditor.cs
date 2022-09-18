using UnityEngine;
using UnityEditor;

namespace LooCast.Noise.Editor
{
    [CustomEditor(typeof(PerlinMapGenerator))]
    public class MapGeneratorEditor : UnityEditor.Editor
    {
        public override void OnInspectorGUI()
        {
            PerlinMapGenerator perlinMapGenerator = (PerlinMapGenerator)target;

            if(DrawDefaultInspector())
            {
                if (perlinMapGenerator.autoUpdate)
                {
                    perlinMapGenerator.GenerateMap();
                }
            }

            if (GUILayout.Button("Generate"))
            {
                perlinMapGenerator.GenerateMap();
            }
        }
    }
}