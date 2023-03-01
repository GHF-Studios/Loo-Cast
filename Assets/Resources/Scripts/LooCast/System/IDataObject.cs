﻿using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identification;

    public interface IDataObject : IData, IDataObjectIdentifiable
    {
        #region Properties
        public IDataObjectType DataObjectType { get; }
        public IDataObject? ParentDataObject { get; }
        public IDataFile? ParentDataFile { get; }
        public SerializableList<IDataObject> ChildDataObjects { get; }
        #endregion
    }
}
