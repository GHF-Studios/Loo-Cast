using UnityEditor;
using UnityEngine;

namespace LooCast.Variable.Editor
{
    using LooCast.Util;

    [CustomPropertyDrawer(typeof(BoolVariable))]
    public class BoolVariableDrawer : PropertyDrawer
    {
        public override void OnGUI(Rect position, SerializedProperty property, GUIContent label)
        {
            EditorGUI.BeginProperty(position, label, property);

            position = EditorGUI.PrefixLabel(position, GUIUtility.GetControlID(FocusType.Passive), label);

            BoolVariable boolVariable = (BoolVariable)PropertyDrawerUtil.GetTargetObjectOfProperty(property);
            bool value = EditorGUI.Toggle(position, boolVariable.Value);
            boolVariable.Value = value;

            EditorGUI.EndProperty();
        }
    } 
}
