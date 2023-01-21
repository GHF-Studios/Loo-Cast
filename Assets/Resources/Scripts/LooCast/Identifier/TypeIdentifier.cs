using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Identifier
{
    using Util.Collections.Generic;
    
    [Serializable]
    public class TypeIdentifier : IIdentifiableType
    {
        #region Properties
        public string ID
        {
            get
            {
                return $"{TypeNamespace.ID}.{ParentType.ID}.{TypeName}";
            }
        }

        public string TypeName => typeName;
        public IIdentifiableType ParentType => parentType;
        public List<IIdentifiableType> ChildTypes => childTypes.Values;
        public IIdentifiableNamespace TypeNamespace => typeNamespace;
        #endregion

        #region Fields
        [SerializeField] private string typeName;
        [SerializeField] private IIdentifiableType parentType;
        [SerializeField] private SerializableList<IIdentifiableType> childTypes;
        [SerializeField] private IIdentifiableNamespace typeNamespace;
        #endregion

        #region Methods
        public void AddChildType(IIdentifiableType childType)
        {
            throw new NotImplementedException();
        }
        #endregion
    }
}