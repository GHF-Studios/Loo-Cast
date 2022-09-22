using UnityEngine;
using UnityEditor;

namespace LooCast.Test.Editor
{
    [CustomEditor(typeof(UniverseGenerator))]
    public class UniverseGeneratorEditor : UnityEditor.Editor
    {
        private SerializedProperty DEV_UNIVERSE_universeGenerationSettings;

        private SerializedProperty DEV_FILAMENT_filamentPosition;

        private SerializedProperty DEV_SECTOR_filamentPosition;
        private SerializedProperty DEV_SECTOR_sectorPosition;

        private SerializedProperty DEV_REGION_sectorPosition;
        private SerializedProperty DEV_REGION_regionPosition;

        private void OnEnable()
        {
            DEV_UNIVERSE_universeGenerationSettings = serializedObject.FindProperty("DEV_UNIVERSE_universeGenerationSettings");

            DEV_FILAMENT_filamentPosition = serializedObject.FindProperty("DEV_FILAMENT_filamentPosition");

            DEV_SECTOR_filamentPosition = serializedObject.FindProperty("DEV_SECTOR_filamentPosition");
            DEV_SECTOR_sectorPosition = serializedObject.FindProperty("DEV_SECTOR_sectorPosition");

            DEV_REGION_sectorPosition = serializedObject.FindProperty("DEV_REGION_sectorPosition");
            DEV_REGION_regionPosition = serializedObject.FindProperty("DEV_REGION_regionPosition");
        }

        public override void OnInspectorGUI()
        {
            UniverseGenerator universeGenerator = (UniverseGenerator)target;

            universeGenerator.DEV_UNIVERSE_showSection = EditorGUILayout.Foldout(universeGenerator.DEV_UNIVERSE_showSection, "DEV Universe");
            if (universeGenerator.DEV_UNIVERSE_showSection)
            {
                EditorGUILayout.PropertyField(DEV_UNIVERSE_universeGenerationSettings);

                if (GUILayout.Button("Generate"))
                {
                    universeGenerator.GenerateUniverse();
                }

                if (GUILayout.Button("Save"))
                {
                    universeGenerator.SaveUniverse();
                }

                if (GUILayout.Button("Load"))
                {
                    universeGenerator.LoadUniverse();
                }

                if (GUILayout.Button("Unload"))
                {
                    universeGenerator.UnloadUniverse();
                }
            }

            universeGenerator.DEV_FILAMENT_showSection = EditorGUILayout.Foldout(universeGenerator.DEV_FILAMENT_showSection, "DEV Filament");
            if (universeGenerator.DEV_FILAMENT_showSection)
            {
                EditorGUILayout.PropertyField(DEV_FILAMENT_filamentPosition);


            }

            universeGenerator.DEV_SECTOR_showSection = EditorGUILayout.Foldout(universeGenerator.DEV_SECTOR_showSection, "DEV Sector");
            if (universeGenerator.DEV_SECTOR_showSection)
            {
                EditorGUILayout.PropertyField(DEV_SECTOR_filamentPosition);
                EditorGUILayout.PropertyField(DEV_SECTOR_sectorPosition);


            }

            universeGenerator.DEV_REGION_showSection = EditorGUILayout.Foldout(universeGenerator.DEV_REGION_showSection, "DEV Region");
            if (universeGenerator.DEV_REGION_showSection)
            {
                EditorGUILayout.PropertyField(DEV_REGION_sectorPosition);
                EditorGUILayout.PropertyField(DEV_REGION_regionPosition);


            }
        }
    }
}