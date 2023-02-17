namespace LooCast.System.Identification
{
    public interface IUnityInstanceIdentifiable : IInstanceIdentifiable
    {
        #region Properties
        IUnityInstanceIdentifier UnityInstanceIdentifier { get; }
        #endregion
    }
}
