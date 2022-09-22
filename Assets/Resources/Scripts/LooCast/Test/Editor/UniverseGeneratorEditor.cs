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

        public override void OnInspectorGUI()
        {
            UniverseGenerator universeGenerator = (UniverseGenerator)target;


            universeGenerator.DEV_UNIVERSE_showSection = EditorGUILayout.Foldout(universeGenerator.DEV_UNIVERSE_showSection, "Universe");
            if (universeGenerator.DEV_UNIVERSE_showSection)
            {
                EditorGUILayout.PropertyField(DEV_UNIVERSE_universeGenerationSettings, new GUIContent("Universe Generation Settings"));


            }

            universeGenerator.DEV_FILAMENT_showSection = EditorGUILayout.Foldout(universeGenerator.DEV_FILAMENT_showSection, "Filament");
            if (universeGenerator.DEV_FILAMENT_showSection)
            {
                EditorGUILayout.PropertyField(DEV_FILAMENT_filamentPosition, new GUIContent("Filament Position"));
                EditorGUILayout.PropertyField(DEV_FILAMENT_filamentPositions, new GUIContent("Filament Positions"));


            }

            universeGenerator.DEV_SECTOR_showSection = EditorGUILayout.Foldout(universeGenerator.DEV_SECTOR_showSection, "Sector");
            if (universeGenerator.DEV_SECTOR_showSection)
            {
                EditorGUILayout.PropertyField(DEV_SECTOR_filamentPosition, new GUIContent("Filament Position"));
                EditorGUILayout.PropertyField(DEV_SECTOR_sectorPosition, new GUIContent("Sector Position"));
                EditorGUILayout.PropertyField(DEV_SECTOR_sectorPositions, new GUIContent("Sector Positions"));


            }

            universeGenerator.DEV_REGION_showSection = EditorGUILayout.Foldout(universeGenerator.DEV_REGION_showSection, "Region");
            if (universeGenerator.DEV_REGION_showSection)
            {
                EditorGUILayout.PropertyField(DEV_REGION_sectorPosition, new GUIContent("Sector Position"));
                EditorGUILayout.PropertyField(DEV_REGION_regionPosition, new GUIContent("Region Position"));
                EditorGUILayout.PropertyField(DEV_REGION_regionPositions, new GUIContent("Region Positions"));


            }
        }
    }
}