using UnityEditor;
using UnityEngine;

namespace LooCast.Variable.Editor
{
    using LooCast.Util;

    [CustomPropertyDrawer(typeof(StringVariable))]
    public class StringVariableDrawer : PropertyDrawer
    {
        public override void OnGUI(Rect position, SerializedProperty property, GUIContent label)
        {
            EditorGUI.BeginProperty(position, label, property);

            position = EditorGUI.PrefixLabel(position, GUIUtility.GetControlID(FocusType.Passive), label);

            StringVariable stringVariable = (StringVariable)PropertyDrawerUtil.GetTargetObjectOfProperty(property);
            string value = EditorGUI.TextField(position, stringVariable.Value);
            stringVariable.Value = value;

            EditorGUI.EndProperty();
        }
    } 
}
