using UnityEditor;
using UnityEngine;

namespace LooCast.Variable.Editor
{
    using LooCast.Util;

    [CustomPropertyDrawer(typeof(FloatVariable))]
    public class FloatVariableDrawer : PropertyDrawer
    {
        public override void OnGUI(Rect position, SerializedProperty property, GUIContent label)
        {
            EditorGUI.BeginProperty(position, label, property);

            position = EditorGUI.PrefixLabel(position, GUIUtility.GetControlID(FocusType.Passive), label);

            FloatVariable floatVariable = (FloatVariable)PropertyDrawerUtil.GetTargetObjectOfProperty(property);
            float value = EditorGUI.FloatField(position, floatVariable.Value);
            floatVariable.Value = value;

            EditorGUI.EndProperty();
        }
    }
}
