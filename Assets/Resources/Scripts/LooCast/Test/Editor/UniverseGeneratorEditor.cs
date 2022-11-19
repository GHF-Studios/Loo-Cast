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
        private SerializedProperty DEV_FILAMENT_filamentPositionRangeMin;
        private SerializedProperty DEV_FILAMENT_filamentPositionRangeMax;

        private SerializedProperty DEV_SECTOR_filamentPosition;
        private SerializedProperty DEV_SECTOR_sectorPosition;
        private SerializedProperty DEV_SECTOR_sectorPositions;
        private SerializedProperty DEV_SECTOR_sectorPositionRangeMin;
        private SerializedProperty DEV_SECTOR_sectorPositionRangeMax;

        private SerializedProperty DEV_REGION_sectorPosition;
        private SerializedProperty DEV_REGION_regionPosition;
        private SerializedProperty DEV_REGION_regionPositions;
        private SerializedProperty DEV_REGION_regionPositionRangeMin;
        private SerializedProperty DEV_REGION_regionPositionRangeMax;

        private void OnEnable()
        {
            DEV_UNIVERSE_universeGenerationSettings = serializedObject.FindProperty("DEV_UNIVERSE_universeGenerationSettings");

            DEV_FILAMENT_filamentPosition = serializedObject.FindProperty("DEV_FILAMENT_filamentPosition");
            DEV_FILAMENT_filamentPositions = serializedObject.FindProperty("DEV_FILAMENT_filamentPositions");
            DEV_FILAMENT_filamentPositionRangeMin = serializedObject.FindProperty("DEV_FILAMENT_filamentPositionRangeMin");
            DEV_FILAMENT_filamentPositionRangeMax = serializedObject.FindProperty("DEV_FILAMENT_filamentPositionRangeMax");

            DEV_SECTOR_filamentPosition = serializedObject.FindProperty("DEV_SECTOR_filamentPosition");
            DEV_SECTOR_sectorPosition = serializedObject.FindProperty("DEV_SECTOR_sectorPosition");
            DEV_SECTOR_sectorPositions = serializedObject.FindProperty("DEV_SECTOR_sectorPositions");
            DEV_SECTOR_sectorPositionRangeMin = serializedObject.FindProperty("DEV_SECTOR_sectorPositionRangeMin");
            DEV_SECTOR_sectorPositionRangeMax = serializedObject.FindProperty("DEV_SECTOR_sectorPositionRangeMax");

            DEV_REGION_sectorPosition = serializedObject.FindProperty("DEV_REGION_sectorPosition");
            DEV_REGION_regionPosition = serializedObject.FindProperty("DEV_REGION_regionPosition");
            DEV_REGION_regionPositions = serializedObject.FindProperty("DEV_REGION_regionPositions");
            DEV_REGION_regionPositionRangeMin = serializedObject.FindProperty("DEV_REGION_regionPositionRangeMin");
            DEV_REGION_regionPositionRangeMax = serializedObject.FindProperty("DEV_REGION_regionPositionRangeMax");
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
                EditorGUILayout.PropertyField(DEV_FILAMENT_filamentPositionRangeMin, new GUIContent("Filament Position Range Min"));
                EditorGUILayout.PropertyField(DEV_FILAMENT_filamentPositionRangeMax, new GUIContent("Filament Position Range Max"));

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

                if (GUILayout.Button("Generate Filament Range"))
                {
                    universeGenerator.GenerateFilamentRange();
                }

                EditorGUILayout.EndHorizontal();
                #endregion

                EditorGUILayout.Space();

                #region Saving
                EditorGUILayout.BeginHorizontal();

                EditorGUILayout.BeginVertical();
                if (GUILayout.Button("Save Filament"))
                {
                    universeGenerator.SaveFilament();
                }

                if (GUILayout.Button("Save Filaments"))
                {
                    universeGenerator.SaveFilaments();
                }
                EditorGUILayout.EndVertical();

                EditorGUILayout.BeginVertical();
                if (GUILayout.Button("Save All Filaments"))
                {
                    universeGenerator.SaveAllFilaments();
                }

                if (GUILayout.Button("Save Filament Range"))
                {
                    universeGenerator.SaveFilamentRange();
                }
                EditorGUILayout.EndVertical();

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

                if (GUILayout.Button("Load Filament Range"))
                {
                    universeGenerator.LoadFilamentRange();
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

                if (GUILayout.Button("Unload Filament Range"))
                {
                    universeGenerator.UnloadFilamentRange();
                }
                EditorGUILayout.EndVertical();

                EditorGUILayout.EndHorizontal();
                #endregion

                EditorGUILayout.Space();

                #region Deletion
                EditorGUILayout.BeginHorizontal();

                EditorGUILayout.BeginVertical();
                if (GUILayout.Button("Delete Filament"))
                {
                    universeGenerator.DeleteFilament();
                }

                if (GUILayout.Button("Delete Filaments"))
                {
                    universeGenerator.DeleteFilaments();
                }
                EditorGUILayout.EndVertical();

                EditorGUILayout.BeginVertical();
                if (GUILayout.Button("Delete All Filaments"))
                {
                    universeGenerator.DeleteAllFilaments();
                }

                if (GUILayout.Button("Delete Filament Range"))
                {
                    universeGenerator.DeleteFilamentRange();
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
                EditorGUILayout.PropertyField(DEV_SECTOR_sectorPositionRangeMin, new GUIContent("Sector Position Range Min"));
                EditorGUILayout.PropertyField(DEV_SECTOR_sectorPositionRangeMax, new GUIContent("Sector Position Range Max"));

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

                if (GUILayout.Button("Generate Sector Range"))
                {
                    universeGenerator.GenerateSectorRange();
                }

                EditorGUILayout.EndHorizontal();
                #endregion

                EditorGUILayout.Space();

                #region Saving
                EditorGUILayout.BeginHorizontal();

                EditorGUILayout.BeginVertical();
                if (GUILayout.Button("Save Sector"))
                {
                    universeGenerator.SaveSector();
                }

                if (GUILayout.Button("Save Sectors"))
                {
                    universeGenerator.SaveSectors();
                }
                EditorGUILayout.EndVertical();

                EditorGUILayout.BeginVertical();
                if (GUILayout.Button("Save All Sectors"))
                {
                    universeGenerator.SaveAllSectors();
                }

                if (GUILayout.Button("Save Sector Range"))
                {
                    universeGenerator.SaveSectorRange();
                }
                EditorGUILayout.EndVertical();

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

                if (GUILayout.Button("Load Sector Range"))
                {
                    universeGenerator.LoadSectorRange();
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

                if (GUILayout.Button("Unload Sector Range"))
                {
                    universeGenerator.UnloadSectorRange();
                }
                EditorGUILayout.EndVertical();

                EditorGUILayout.EndHorizontal();
                #endregion

                EditorGUILayout.Space();

                #region Deletion
                EditorGUILayout.BeginHorizontal();

                EditorGUILayout.BeginVertical();
                if (GUILayout.Button("Delete Sector"))
                {
                    universeGenerator.DeleteSector();
                }

                if (GUILayout.Button("Delete Sectors"))
                {
                    universeGenerator.DeleteSectors();
                }
                EditorGUILayout.EndVertical();

                EditorGUILayout.BeginVertical();
                if (GUILayout.Button("Delete All Sectors"))
                {
                    universeGenerator.DeleteAllSectors();
                }

                if (GUILayout.Button("Delete Sector Range"))
                {
                    universeGenerator.DeleteSectorRange();
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
                EditorGUILayout.PropertyField(DEV_REGION_regionPositionRangeMin, new GUIContent("Region Position Range Min"));
                EditorGUILayout.PropertyField(DEV_REGION_regionPositionRangeMax, new GUIContent("Region Position Range Max"));

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

                if (GUILayout.Button("Generate Region Range"))
                {
                    universeGenerator.GenerateRegionRange();
                }

                EditorGUILayout.EndHorizontal();
                #endregion

                EditorGUILayout.Space();

                #region Saving
                EditorGUILayout.BeginHorizontal();

                EditorGUILayout.BeginVertical();
                if (GUILayout.Button("Save Region"))
                {
                    universeGenerator.SaveRegion();
                }

                if (GUILayout.Button("Save Regions"))
                {
                    universeGenerator.SaveRegions();
                }
                EditorGUILayout.EndVertical();

                EditorGUILayout.BeginVertical();
                if (GUILayout.Button("Save All Regions"))
                {
                    universeGenerator.SaveAllRegions();
                }

                if (GUILayout.Button("Save Region Range"))
                {
                    universeGenerator.SaveRegionRange();
                }
                EditorGUILayout.EndVertical();

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

                if (GUILayout.Button("Load Region Range"))
                {
                    universeGenerator.LoadRegionRange();
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

                if (GUILayout.Button("Unload Region Range"))
                {
                    universeGenerator.UnloadRegionRange();
                }
                EditorGUILayout.EndVertical();

                EditorGUILayout.EndHorizontal();
                #endregion

                EditorGUILayout.Space();

                #region Deletion
                EditorGUILayout.BeginHorizontal();

                EditorGUILayout.BeginVertical();
                if (GUILayout.Button("Delete Region"))
                {
                    universeGenerator.DeleteRegion();
                }

                if (GUILayout.Button("Delete Regions"))
                {
                    universeGenerator.DeleteRegions();
                }
                EditorGUILayout.EndVertical();

                EditorGUILayout.BeginVertical();
                if (GUILayout.Button("Delete All Regions"))
                {
                    universeGenerator.DeleteAllRegions();
                }

                if (GUILayout.Button("Delete Region Range"))
                {
                    universeGenerator.DeleteRegionRange();
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