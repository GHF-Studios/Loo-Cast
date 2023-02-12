using System;
using UnityEngine;

namespace LooCast.Identifier
{
    [Serializable]
    public struct TypeIdentifier : IIdentifier
    {
        #region Properties
        public NamespaceIdentifier TypeNamespace => typeNamespace;
        public Type SystemType => systemType;
        public string TypeID => typeNamespace.NamespaceID + ":" + SystemType.Name;
        public string ID => TypeID;
        #endregion

        #region Fields
        [SerializeField] private NamespaceIdentifier typeNamespace;
        [SerializeField] private Type systemType;
        #endregion

        #region Constructors
        internal TypeIdentifier(NamespaceIdentifier typeNamespace, Type systemType)
        {
            this.typeNamespace = typeNamespace;
            this.systemType = systemType;
        }
        #endregion

        #region Overrides
        // TODO: Implement overrides
        #endregion
    }
}
