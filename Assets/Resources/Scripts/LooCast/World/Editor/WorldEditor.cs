using System;
using UnityEditor;
using UnityEngine;

namespace LooCast.World.Editor
{
    [CustomEditor(typeof(World))]
    public class WorldEditor : UnityEditor.Editor
    {
        public override void OnInspectorGUI()
        {
            World world = (World)target;
            DrawDefaultInspector();

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
