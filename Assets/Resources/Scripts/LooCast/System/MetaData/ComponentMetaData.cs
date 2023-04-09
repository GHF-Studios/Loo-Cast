using System;
using UnityEngine;

namespace LooCast.System.MetaData
{
    using global::LooCast.System;
    using global::LooCast.System.Identifiers;

    [Serializable]
    public class ComponentMetaData : MetaData
    {
        #region Properties
        public TypeIdentifier TypeIdentifier => typeIdentifier;
        public GameObject ContainingGameObject => containingGameObject;
#nullable enable
        public global::System.Collections.Generic.List<MetaData>? Dependencies
        {
            get
            {
                return dependencies;
            }
            protected set
            {
                dependencies = value;
            }
        }
#nullable disable
        #endregion

        #region Fields
        [SerializeField] private TypeIdentifier typeIdentifier;
        [SerializeField] private GameObject containingGameObject;
#nullable enable
        [SerializeField] private global::System.Collections.Generic.List<MetaData>? dependencies;
#nullable disable
        #endregion

        #region Constructors
#nullable enable
        public ComponentMetaData(TypeIdentifier typeIdentifier, GameObject containingGameObject, global::System.Collections.Generic.List<MetaData>? dependencies = null)
        {
            this.typeIdentifier = typeIdentifier;
            this.containingGameObject = containingGameObject;
            this.dependencies = dependencies;
        }
#nullable disable
        #endregion

        #region Methods
        public virtual void Validate()
        {
            if (typeIdentifier == null)
            {
                throw new ArgumentNullException(nameof(typeIdentifier));
            }
            
            if (containingGameObject == null)
            {
                throw new ArgumentNullException(nameof(containingGameObject));
            }
        }
        #endregion
    }
}
