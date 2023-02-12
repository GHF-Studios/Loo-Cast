using System;
using UnityEngine;

namespace LooCast.Identifier
{
    [Serializable]
    public struct NamespaceIdentifier : IIdentifier
    {
        #region Properties
        public string NamespaceID => namespaceID;
        public string ID => NamespaceID;
        #endregion

        #region Fields
        [SerializeField] private string namespaceID;
        #endregion

        #region Constructors
        internal NamespaceIdentifier(string namespaceID)
        {
            this.namespaceID = namespaceID;
        }
        #endregion

        #region Overrides
        // TODO: Implement overrides
        #endregion
    }
}
