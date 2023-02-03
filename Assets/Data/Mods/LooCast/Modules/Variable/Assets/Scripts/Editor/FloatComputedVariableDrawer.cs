using UnityEditor;
using UnityEngine;

namespace LooCast.Variable.Editor
{
    using LooCast.Util.Editor;

    [CustomPropertyDrawer(typeof(FloatComputedVariable))]
    public class FloatComputedVariableDrawer : PropertyDrawer
    {
        public override void OnGUI(Rect position, SerializedProperty property, GUIContent label)
        {
            EditorGUI.BeginProperty(position, label, property);

            position.yMax -= 24.0f;
            position = EditorGUI.PrefixLabel(position, GUIUtility.GetControlID(FocusType.Passive), label);
            FloatComputedVariable floatVariable = (FloatComputedVariable)PropertyDrawerUtil.GetTargetObjectOfProperty(property);
            floatVariable.BaseValue = EditorGUI.FloatField(position, floatVariable.BaseValue);

            if (floatVariable.IsInitialized)
            {
                position.position += Vector2.up * 18.0f;
                EditorGUI.LabelField(position, "Evaluation:\t" + floatVariable.Value.ToString());
            }
            else
            {
                position.position += Vector2.up * 18.0f;
                EditorGUI.LabelField(position, "Evaluation:\t" + "N/A");
            }

            EditorGUI.EndProperty();
        }

        public override float GetPropertyHeight(SerializedProperty property, GUIContent label)
        {
            return 44.0f;
        }
    }
}
