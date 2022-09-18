using UnityEngine;
using UnityEditor;

namespace LooCast.Noise.Editor
{
    [CustomEditor(typeof(VoronoiMapGenerator))]
    public class VoronoiMapGeneratorEditor : UnityEditor.Editor
    {
        public override void OnInspectorGUI()
        {
            VoronoiMapGenerator voronoiMapGenerator = (VoronoiMapGenerator)target;

            if(DrawDefaultInspector())
            {
                if (voronoiMapGenerator.autoUpdate)
                {
                    voronoiMapGenerator.GenerateMap();
                }
            }

            if (GUILayout.Button("Generate"))
            {
                voronoiMapGenerator.GenerateMap();
            }
        }
    }
}