using UnityEngine;
using UnityEditor;

namespace LooCast.Test.Editor
{
    [CustomEditor(typeof(PerlinMapGeneratorGPU))]
    public class PerlinMapGeneratorGPUEditor : UnityEditor.Editor
    {
        private SerializedProperty CurrentDrawMode;
        private SerializedProperty MapWidth;
        private SerializedProperty MapHeight;
        private SerializedProperty NoiseScale;
        private SerializedProperty NoiseAmplitude;
        private SerializedProperty Octaves;
        private SerializedProperty Persistence;
        private SerializedProperty Lacunarity;
        private SerializedProperty Seed;
        private SerializedProperty Offset;
        private SerializedProperty AutoUpdate;
        private SerializedProperty Regions;
        private SerializedProperty ComputeShader;

        private void OnEnable()
        {
            CurrentDrawMode = serializedObject.FindProperty("CurrentDrawMode");
            MapWidth = serializedObject.FindProperty("MapWidth");
            MapHeight = serializedObject.FindProperty("MapHeight");
            NoiseScale = serializedObject.FindProperty("NoiseScale");
            NoiseAmplitude = serializedObject.FindProperty("NoiseAmplitude");
            Octaves = serializedObject.FindProperty("Octaves");
            Persistence = serializedObject.FindProperty("Persistence");
            Lacunarity = serializedObject.FindProperty("Lacunarity");
            Seed = serializedObject.FindProperty("Seed");
            Offset = serializedObject.FindProperty("Offset");
            AutoUpdate = serializedObject.FindProperty("AutoUpdate");
            Regions = serializedObject.FindProperty("Regions");
            ComputeShader = serializedObject.FindProperty("ComputeShader");
        }
        
        public override void OnInspectorGUI()
        {
            PerlinMapGeneratorGPU perlinMapGeneratorGPU = (PerlinMapGeneratorGPU)target;

            EditorGUI.BeginChangeCheck();

            EditorGUILayout.PropertyField(CurrentDrawMode);
            EditorGUILayout.PropertyField(MapWidth);
            EditorGUILayout.PropertyField(MapHeight);
            EditorGUILayout.PropertyField(NoiseScale);
            EditorGUILayout.PropertyField(NoiseAmplitude);
            EditorGUILayout.PropertyField(Octaves);
            EditorGUILayout.PropertyField(Persistence);
            EditorGUILayout.PropertyField(Lacunarity);
            EditorGUILayout.PropertyField(Seed);
            EditorGUILayout.PropertyField(Offset);
            EditorGUILayout.PropertyField(AutoUpdate);
            EditorGUILayout.PropertyField(Regions);
            EditorGUILayout.PropertyField(ComputeShader);

            if(EditorGUI.EndChangeCheck())
            {
                if (perlinMapGeneratorGPU.AutoUpdate && perlinMapGeneratorGPU.gameObject.activeInHierarchy)
                {
                    perlinMapGeneratorGPU.GenerateMap();
                }
            }

            if (GUILayout.Button("Generate"))
            {
                perlinMapGeneratorGPU.GenerateMap();
            }

            serializedObject.ApplyModifiedProperties();
        }
    }
}