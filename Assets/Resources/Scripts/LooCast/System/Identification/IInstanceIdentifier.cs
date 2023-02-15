﻿namespace LooCast.System.Identification
{
    public interface IInstanceIdentifier : IGenericIdentifier<Instance>
    {
        #region Properties
        string InstanceTypeID { get; }
        string InstanceGUID { get; }
        string InstanceID { get; }
        #endregion
    }
}