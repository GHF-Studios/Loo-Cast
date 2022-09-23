using UnityEngine;
using UnityEditor;

namespace LooCast.Test.Editor
{
    [CustomEditor(typeof(UniverseGenerator))]
    public class UniverseGeneratorEditor : UnityEditor.Editor
    {
        private SerializedProperty DEV_UNIVERSE_universeGenerationSettings;

        private SerializedProperty DEV_FILAMENT_filamentPosition;
        private SerializedProperty DEV_FILAMENT_filamentPositions;

        private SerializedProperty DEV_SECTOR_filamentPosition;
        private SerializedProperty DEV_SECTOR_sectorPosition;
        private SerializedProperty DEV_SECTOR_sectorPositions;

        private SerializedProperty DEV_REGION_sectorPosition;
        private SerializedProperty DEV_REGION_regionPosition;
        private SerializedProperty DEV_REGION_regionPositions;

        private void OnEnable()
        {
            DEV_UNIVERSE_universeGenerationSettings = serializedObject.FindProperty("DEV_UNIVERSE_universeGenerationSettings");

            DEV_FILAMENT_filamentPosition = serializedObject.FindProperty("DEV_FILAMENT_filamentPosition");
            DEV_FILAMENT_filamentPositions = serializedObject.FindProperty("DEV_FILAMENT_filamentPositions");

            DEV_SECTOR_filamentPosition = serializedObject.FindProperty("DEV_SECTOR_filamentPosition");
            DEV_SECTOR_sectorPosition = serializedObject.FindProperty("DEV_SECTOR_sectorPosition");
            DEV_SECTOR_sectorPositions = serializedObject.FindProperty("DEV_SECTOR_sectorPositions");

            DEV_REGION_sectorPosition = serializedObject.FindProperty("DEV_REGION_sectorPosition");
            DEV_REGION_regionPosition = serializedObject.FindProperty("DEV_REGION_regionPosition");
            DEV_REGION_regionPositions = serializedObject.FindProperty("DEV_REGION_regionPositions");
        }

        //Add Left Padding to Foldout Contents and Try GUILayout
        public override void OnInspectorGUI()
        {
            UniverseGenerator universeGenerator = (UniverseGenerator)target;

            universeGenerator.DEV_UNIVERSE_showSection = EditorGUILayout.Foldout(universeGenerator.DEV_UNIVERSE_showSection, "Universe");
            if (universeGenerator.DEV_UNIVERSE_showSection)
            {
                EditorGUILayout.BeginHorizontal();

                EditorGUILayout.Space(-20.0f);

                EditorGUILayout.BeginVertical();

                EditorGUILayout.PropertyField(DEV_UNIVERSE_universeGenerationSettings, new GUIContent("Universe Generation Settings"));
                
                if (GUILayout.Button("Generate Universe"))
                {
                    universeGenerator.GenerateUniverse();
                }

                if (GUILayout.Button("Save Universe"))
                {
                    universeGenerator.SaveUniverse();
                }

                if (GUILayout.Button("Load Universe"))
                {
                    universeGenerator.LoadUniverse();
                }

                if (GUILayout.Button("Unload Universe"))
                {
                    universeGenerator.UnloadUniverse();
                }

                if (GUILayout.Button("Delete Universe"))
                {
                    universeGenerator.DeleteUniverse();
                }

                EditorGUILayout.EndVertical();

                EditorGUILayout.EndHorizontal();
            }

            universeGenerator.DEV_FILAMENT_showSection = EditorGUILayout.Foldout(universeGenerator.DEV_FILAMENT_showSection, "Filament");
            if (universeGenerator.DEV_FILAMENT_showSection)
            {
                EditorGUILayout.BeginHorizontal();

                EditorGUILayout.Space();

                EditorGUILayout.BeginVertical();

                EditorGUILayout.PropertyField(DEV_FILAMENT_filamentPosition, new GUIContent("Filament Position"));
                EditorGUILayout.PropertyField(DEV_FILAMENT_filamentPositions, new GUIContent("Filament Positions"));

                #region Generation
                EditorGUILayout.BeginHorizontal();

                if (GUILayout.Button("Generate Filament"))
                {
                    universeGenerator.GenerateFilament();
                }

                if (GUILayout.Button("Generate Filaments"))
                {
                    universeGenerator.GenerateFilaments();
                }

                EditorGUILayout.EndHorizontal();
                #endregion

                EditorGUILayout.Space();

                #region Saving
                EditorGUILayout.BeginHorizontal();

                if (GUILayout.Button("Save Filament"))
                {
                    universeGenerator.SaveFilament();
                }

                if (GUILayout.Button("Save Filaments"))
                {
                    universeGenerator.SaveFilaments();
                }

                if (GUILayout.Button("Save All Filaments"))
                {
                    universeGenerator.SaveAllFilaments();
                }

                EditorGUILayout.EndHorizontal();
                #endregion

                EditorGUILayout.Space();

                #region Loading
                EditorGUILayout.BeginHorizontal();

                EditorGUILayout.BeginVertical();
                if (GUILayout.Button("Load Filament"))
                {
                    universeGenerator.LoadFilament();
                }

                if (GUILayout.Button("Load Filaments"))
                {
                    universeGenerator.LoadFilaments();
                }
                EditorGUILayout.EndVertical();

                EditorGUILayout.BeginVertical();
                if (GUILayout.Button("Unload Filament"))
                {
                    universeGenerator.UnloadFilament();
                }

                if (GUILayout.Button("Unload Filaments"))
                {
                    universeGenerator.UnloadFilaments();
                }

                if (GUILayout.Button("Unload All Filaments"))
                {
                    universeGenerator.UnloadAllFilaments();
                }
                EditorGUILayout.EndVertical();

                EditorGUILayout.EndHorizontal();
                #endregion

                EditorGUILayout.Space();

                #region Deletion
                EditorGUILayout.BeginHorizontal();

                if (GUILayout.Button("Delete Filament"))
                {
                    universeGenerator.DeleteFilament();
                }

                if (GUILayout.Button("Delete Filaments"))
                {
                    universeGenerator.DeleteFilaments();
                }

                if (GUILayout.Button("Delete All Filaments"))
                {
                    universeGenerator.DeleteAllFilaments();
                }

                EditorGUILayout.EndHorizontal();
                #endregion

                EditorGUILayout.Space();

                #region Spawning
                EditorGUILayout.BeginHorizontal();

                EditorGUILayout.BeginVertical();
                if (GUILayout.Button("Spawn Filament"))
                {
                    universeGenerator.SpawnFilament();
                }

                if (GUILayout.Button("Spawn Filaments"))
                {
                    universeGenerator.SpawnFilaments();
                }
                EditorGUILayout.EndVertical();

                EditorGUILayout.BeginVertical();
                if (GUILayout.Button("Despawn Filament"))
                {
                    universeGenerator.DespawnFilament();
                }

                if (GUILayout.Button("Despawn Filaments"))
                {
                    universeGenerator.DespawnFilaments();
                }
                EditorGUILayout.EndVertical();

                EditorGUILayout.EndHorizontal();
                #endregion

                EditorGUILayout.EndVertical();

                EditorGUILayout.EndHorizontal();
            }

            universeGenerator.DEV_SECTOR_showSection = EditorGUILayout.Foldout(universeGenerator.DEV_SECTOR_showSection, "Sector");
            if (universeGenerator.DEV_SECTOR_showSection)
            {
                EditorGUILayout.BeginHorizontal();

                EditorGUILayout.Space();

                EditorGUILayout.BeginVertical();

                EditorGUILayout.PropertyField(DEV_SECTOR_filamentPosition, new GUIContent("Filament Position"));
                EditorGUILayout.PropertyField(DEV_SECTOR_sectorPosition, new GUIContent("Sector Position"));
                EditorGUILayout.PropertyField(DEV_SECTOR_sectorPositions, new GUIContent("Sector Positions"));

                #region Generation
                EditorGUILayout.BeginHorizontal();

                if (GUILayout.Button("Generate Sector"))
                {
                    universeGenerator.GenerateSector();
                }

                if (GUILayout.Button("Generate Sectors"))
                {
                    universeGenerator.GenerateSectors();
                }

                EditorGUILayout.EndHorizontal();
                #endregion

                EditorGUILayout.Space();

                #region Saving
                EditorGUILayout.BeginHorizontal();

                if (GUILayout.Button("Save Sector"))
                {
                    universeGenerator.SaveSector();
                }

                if (GUILayout.Button("Save Sectors"))
                {
                    universeGenerator.SaveSectors();
                }

                if (GUILayout.Button("Save All Sectors"))
                {
                    universeGenerator.SaveAllSectors();
                }

                EditorGUILayout.EndHorizontal();
                #endregion

                EditorGUILayout.Space();

                #region Loading
                EditorGUILayout.BeginHorizontal();

                EditorGUILayout.BeginVertical();
                if (GUILayout.Button("Load Sector"))
                {
                    universeGenerator.LoadSector();
                }

                if (GUILayout.Button("Load Sectors"))
                {
                    universeGenerator.LoadSectors();
                }
                EditorGUILayout.EndVertical();

                EditorGUILayout.BeginVertical();
                if (GUILayout.Button("Unload Sector"))
                {
                    universeGenerator.UnloadSector();
                }

                if (GUILayout.Button("Unload Sectors"))
                {
                    universeGenerator.UnloadSectors();
                }

                if (GUILayout.Button("Unload All Sectors"))
                {
                    universeGenerator.UnloadAllSectors();
                }
                EditorGUILayout.EndVertical();

                EditorGUILayout.EndHorizontal();
                #endregion

                EditorGUILayout.Space();

                #region Deletion
                EditorGUILayout.BeginHorizontal();

                if (GUILayout.Button("Delete Sector"))
                {
                    universeGenerator.DeleteSector();
                }

                if (GUILayout.Button("Delete Sectors"))
                {
                    universeGenerator.DeleteSectors();
                }

                if (GUILayout.Button("Delete All Sectors"))
                {
                    universeGenerator.DeleteAllSectors();
                }

                EditorGUILayout.EndHorizontal();
                #endregion

                EditorGUILayout.Space();

                #region Spawning
                EditorGUILayout.BeginHorizontal();

                EditorGUILayout.BeginVertical();
                if (GUILayout.Button("Spawn Sector"))
                {
                    universeGenerator.SpawnSector();
                }

                if (GUILayout.Button("Spawn Sectors"))
                {
                    universeGenerator.SpawnSectors();
                }
                EditorGUILayout.EndVertical();

                EditorGUILayout.BeginVertical();
                if (GUILayout.Button("Despawn Sector"))
                {
                    universeGenerator.DespawnSector();
                }

                if (GUILayout.Button("Despawn Sectors"))
                {
                    universeGenerator.DespawnSectors();
                }
                EditorGUILayout.EndVertical();

                EditorGUILayout.EndHorizontal();
                #endregion

                EditorGUILayout.EndVertical();

                EditorGUILayout.EndHorizontal();
            }

            universeGenerator.DEV_REGION_showSection = EditorGUILayout.Foldout(universeGenerator.DEV_REGION_showSection, "Region");
            if (universeGenerator.DEV_REGION_showSection)
            {
                EditorGUILayout.BeginHorizontal();

                EditorGUILayout.Space();

                EditorGUILayout.BeginVertical();

                EditorGUILayout.PropertyField(DEV_REGION_sectorPosition, new GUIContent("Sector Position"));
                EditorGUILayout.PropertyField(DEV_REGION_regionPosition, new GUIContent("Region Position"));
                EditorGUILayout.PropertyField(DEV_REGION_regionPositions, new GUIContent("Region Positions"));

                #region Generation
                EditorGUILayout.BeginHorizontal();

                if (GUILayout.Button("Generate Region"))
                {
                    universeGenerator.GenerateRegion();
                }

                if (GUILayout.Button("Generate Regions"))
                {
                    universeGenerator.GenerateRegions();
                }

                EditorGUILayout.EndHorizontal();
                #endregion

                EditorGUILayout.Space();

                #region Saving
                EditorGUILayout.BeginHorizontal();

                if (GUILayout.Button("Save Region"))
                {
                    universeGenerator.SaveRegion();
                }

                if (GUILayout.Button("Save Regions"))
                {
                    universeGenerator.SaveRegions();
                }

                if (GUILayout.Button("Save All Regions"))
                {
                    universeGenerator.SaveAllRegions();
                }

                EditorGUILayout.EndHorizontal();
                #endregion

                EditorGUILayout.Space();

                #region Loading
                EditorGUILayout.BeginHorizontal();

                EditorGUILayout.BeginVertical();
                if (GUILayout.Button("Load Region"))
                {
                    universeGenerator.LoadRegion();
                }

                if (GUILayout.Button("Load Regions"))
                {
                    universeGenerator.LoadRegions();
                }
                EditorGUILayout.EndVertical();

                EditorGUILayout.BeginVertical();
                if (GUILayout.Button("Unload Region"))
                {
                    universeGenerator.UnloadRegion();
                }

                if (GUILayout.Button("Unload Regions"))
                {
                    universeGenerator.UnloadRegions();
                }

                if (GUILayout.Button("Unload All Regions"))
                {
                    universeGenerator.UnloadAllRegions();
                }
                EditorGUILayout.EndVertical();

                EditorGUILayout.EndHorizontal();
                #endregion

                EditorGUILayout.Space();

                #region Deletion
                EditorGUILayout.BeginHorizontal();

                if (GUILayout.Button("Delete Region"))
                {
                    universeGenerator.DeleteRegion();
                }

                if (GUILayout.Button("Delete Regions"))
                {
                    universeGenerator.DeleteRegions();
                }

                if (GUILayout.Button("Delete All Regions"))
                {
                    universeGenerator.DeleteAllRegions();
                }

                EditorGUILayout.EndHorizontal();
                #endregion

                EditorGUILayout.Space();

                #region Spawning
                EditorGUILayout.BeginHorizontal();

                EditorGUILayout.BeginVertical();
                if (GUILayout.Button("Spawn Region"))
                {
                    universeGenerator.SpawnRegion();
                }

                if (GUILayout.Button("Spawn Regions"))
                {
                    universeGenerator.SpawnRegions();
                }
                EditorGUILayout.EndVertical();

                EditorGUILayout.BeginVertical();
                if (GUILayout.Button("Despawn Region"))
                {
                    universeGenerator.DespawnRegion();
                }

                if (GUILayout.Button("Despawn Regions"))
                {
                    universeGenerator.DespawnRegions();
                }
                EditorGUILayout.EndVertical();

                EditorGUILayout.EndHorizontal();
                #endregion

                EditorGUILayout.EndVertical();

                EditorGUILayout.EndHorizontal();
            }

            serializedObject.ApplyModifiedProperties();
        }
    }
}