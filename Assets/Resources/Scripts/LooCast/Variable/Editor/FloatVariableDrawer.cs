using UnityEditor;
using UnityEditorInternal;
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
            floatVariable.Value = EditorGUI.FloatField(position, floatVariable.Value);

            EditorGUI.EndProperty();
        }
    }
}
