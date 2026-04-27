using UnityEngine;
using UnityEditor;

namespace LooCast.Test.Editor
{
    [CustomEditor(typeof(VoronoiMapGeneratorGPU))]
    public class VoronoiMapGeneratorGPUEditor : UnityEditor.Editor
    {
        private SerializedProperty MapWidth;
        private SerializedProperty MapHeight;
        private SerializedProperty SampleCellAmount;
        private SerializedProperty CellSpread;
        private SerializedProperty Power;
        private SerializedProperty Amplitude;
        private SerializedProperty Seed;
        private SerializedProperty AutoUpdate;
        private SerializedProperty ComputeShader;

        private void OnEnable()
        {
            MapWidth = serializedObject.FindProperty("MapWidth");
            MapHeight = serializedObject.FindProperty("MapHeight");
            SampleCellAmount = serializedObject.FindProperty("SampleCellAmount");
            CellSpread = serializedObject.FindProperty("CellSpread");
            Power = serializedObject.FindProperty("Power");
            Amplitude = serializedObject.FindProperty("Amplitude");
            Seed = serializedObject.FindProperty("Seed");
            AutoUpdate = serializedObject.FindProperty("AutoUpdate");
            ComputeShader = serializedObject.FindProperty("ComputeShader");
        }
        public override void OnInspectorGUI()
        {
            VoronoiMapGeneratorGPU voronoiMapGeneratorGPU = (VoronoiMapGeneratorGPU)target;

            EditorGUI.BeginChangeCheck();

            EditorGUILayout.PropertyField(MapWidth);
            EditorGUILayout.PropertyField(MapHeight);
            EditorGUILayout.PropertyField(SampleCellAmount);
            EditorGUILayout.PropertyField(CellSpread);
            EditorGUILayout.PropertyField(Power);
            EditorGUILayout.PropertyField(Amplitude);
            EditorGUILayout.PropertyField(Seed);
            EditorGUILayout.PropertyField(AutoUpdate);
            EditorGUILayout.PropertyField(ComputeShader);

            if(EditorGUI.EndChangeCheck())
            {
                if (voronoiMapGeneratorGPU.AutoUpdate && voronoiMapGeneratorGPU.gameObject.activeInHierarchy)
                {
                    voronoiMapGeneratorGPU.GenerateMap();
                }
            }

            if (GUILayout.Button("Generate"))
            {
                voronoiMapGeneratorGPU.GenerateMap();
            }

            serializedObject.ApplyModifiedProperties();
        }
    }
}