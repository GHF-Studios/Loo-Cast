using LooCast.System.Identification;
using LooCast.System.Types;
using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public class Object : CSharpInstance, IObject
    {
        #region Properties
        public IObjectType ObjectType => objectType;
        public IObject ParentObject => parentObject;
        public List<IObject> ChildObjects => childObjects;
        public IObjectIdentifier ObjectIdentifier => objectIdentifier;
        #endregion

        #region Fields
        private IObjectIdentifier objectIdentifier;
        private IObjectType objectType;
        private IObject parentObject;
        private List<IObject> childObjects;
        #endregion

        #region Constructors
        public Object(object systemObject, IObjectType objectType) : base(systemObject, objectType)
        {

        }

        public Object(object systemObject, IObjectType objectType, IObject parentObject) : base(systemObject, objectType, parentObject)
        {

        }
        #endregion
    }
}
