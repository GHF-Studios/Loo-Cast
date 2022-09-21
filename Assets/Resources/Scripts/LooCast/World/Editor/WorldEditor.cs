using System;
using UnityEditor;
using UnityEngine;

namespace LooCast.World.Editor
{
    [CustomEditor(typeof(World))]
    public class WorldEditor : UnityEditor.Editor
    {
        private bool showFilamentChunksSection = false;
        private bool showChunksSection = false;

        private SerializedProperty filamentChunkSize;
        private SerializedProperty filamentChunkAmount;

        private SerializedProperty chunkSize;
        private SerializedProperty chunkPrefab;
        private SerializedProperty generationSettings;

        private void OnEnable()
        {
            filamentChunkSize = serializedObject.FindProperty("filamentChunkSize");
            filamentChunkAmount = serializedObject.FindProperty("filamentChunkAmount");
            chunkSize = serializedObject.FindProperty("chunkSize");
            chunkPrefab = serializedObject.FindProperty("chunkPrefab");
            generationSettings = serializedObject.FindProperty("generationSettings");
        }

        public override void OnInspectorGUI()
        {
            World world = (World)target;

            showFilamentChunksSection = EditorGUILayout.Foldout(showFilamentChunksSection, "Filament Chunk Section");
            if (showFilamentChunksSection)
            {
                EditorGUILayout.PropertyField(filamentChunkSize);
                EditorGUILayout.PropertyField(filamentChunkAmount);

                if (GUILayout.Button("Generate Filament Chunks"))
                {

                }
            }

            showChunksSection = EditorGUILayout.Foldout(showChunksSection, "Chunk Section");
            if (showChunksSection)
            {
                EditorGUILayout.PropertyField(chunkSize);
                EditorGUILayout.PropertyField(chunkPrefab);
                EditorGUILayout.PropertyField(generationSettings);

                if (GUILayout.Button("Generate Chunks"))
                {
                    world.DEV_GenerateChunks();
                }

                if (GUILayout.Button("Load Chunks"))
                {
                    world.DEV_LoadChunks();
                }

                if (GUILayout.Button("Unload Chunks"))
                {
                    world.DEV_UnloadChunks();
                }

                if (GUILayout.Button("Spawn Chunks"))
                {
                    world.DEV_SpawnChunks();
                }

                if (GUILayout.Button("Despawn Chunks"))
                {
                    world.DEV_DespawnChunks();
                } 
            }
        }
    }
}
