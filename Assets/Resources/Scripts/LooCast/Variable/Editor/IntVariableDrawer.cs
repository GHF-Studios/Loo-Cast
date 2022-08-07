using UnityEditor;
using UnityEngine;

namespace LooCast.Variable.Editor
{
    using LooCast.Util;

    [CustomPropertyDrawer(typeof(IntVariable))]
    public class IntVariableDrawer : PropertyDrawer
    {
        public override void OnGUI(Rect position, SerializedProperty property, GUIContent label)
        {
            EditorGUI.BeginProperty(position, label, property);

            position = EditorGUI.PrefixLabel(position, GUIUtility.GetControlID(FocusType.Passive), label);

            IntVariable intVariable = (IntVariable)PropertyDrawerUtil.GetTargetObjectOfProperty(property);
            int value = EditorGUI.IntField(position, intVariable.Value);
            intVariable.Value = value;

            EditorGUI.EndProperty();
        }
    } 
}
