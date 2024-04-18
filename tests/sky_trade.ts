import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SkyTrade } from "../target/types/sky_trade";

describe("sky_trade", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");

  const program = anchor.workspace.SkyTrade as Program<SkyTrade>;
  const adminOwner = anchor.web3.Keypair.generate();
  const adminDepositAccount = anchor.web3.Keypair.generate();
  const propertyOwner = anchor.web3.Keypair.generate();
  const droneOperator = anchor.web3.Keypair.generate();
  const buyer = anchor.web3.Keypair.generate();
  const seller = anchor.web3.Keypair.generate();

  // adminPdaAuth pda and its bump
  let [adminPdaAuth, adminPdaBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("admin-auth"),
        adminDepositAccount.publicKey.toBuffer(),
      ],
      program.programId
    );

  // adminSolVault system account and its bump
  let [adminSolVault, adminSolBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("admin-sol-vault"),
        adminPdaAuth.toBuffer(),
      ],
      program.programId
    );

  // configs account
  let [configs] = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("configs")],
    program.programId
  );

  // property owner account
  let [propertyOwnerAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("property-owner"),
      propertyOwner.publicKey.toBuffer(),
    ],
    program.programId
  );

  // airspace account
  let [airspace] = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("airspace")],
    program.programId
  );

  // drone operator account
  let [droneOperatorAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("drone-operator"),
      droneOperator.publicKey.toBuffer(),
    ],
    program.programId
  );

  // property account
  let [property] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("property"),
      propertyOwner.publicKey.toBuffer(),
    ],
    program.programId
  );

  // seller property account
  let [sellerProperty] = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("property"), seller.publicKey.toBuffer()],
    program.programId
  );

  // buyer account
  let [buyerAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("property-owner"),
      buyer.publicKey.toBuffer(),
    ],
    program.programId
  );

  // seller account
  let [sellerAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("property-owner"),
      seller.publicKey.toBuffer(),
    ],
    program.programId
  );

  // adminOwner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      adminOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // property owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      propertyOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // drone operator
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      droneOperator.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // buyer
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      buyer.publicKey,
      50 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // seller
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      seller.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  it("Is initialized!", async () => {
    let initParams = {
      // price per cubic foot. Used to calculate income
      // to be generated by property owner i.e 1 Sol
      pricePerCubicFoot: 1,
      buyingPricePerCubicFoot: 5, // buying price per cubic foot i.e 5 Sol
    };

    const tx = await program.methods
      .init(initParams)
      .accounts({
        owner: adminOwner.publicKey,
        configs: configs,
        airspace: airspace,
        adminDepositAccount: adminDepositAccount.publicKey,
        adminPdaAuth: adminPdaAuth,
        adminSolVault: adminSolVault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([adminOwner, adminDepositAccount])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.configs.fetch(configs);
    console.log("configs: ", result);
  });

  it("Is register property owner!", async () => {
    // property owner categories
    /* 1: IndividualPropertyOwner,
        2: RealEstateCompany,
        3: CityMunicipality, */

    let category = 1; // IndividualPropertyOwner

    let initParams = {
      name: "john doe",
      category: category,
      country: "USA",
    };

    const tx = await program.methods
      .registerPropertyOwner(initParams)
      .accounts({
        owner: propertyOwner.publicKey,
        propertyOwner: propertyOwnerAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([propertyOwner])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.propertyOwner.fetch(
      propertyOwnerAccount
    );
    console.log("property owner account: ", result);
  });

  it("Is register drone operator!", async () => {
    // drone operator categories
    /* 1: Individual,
        2: Company, */

    let category = 1; // Individual

    let initParams = {
      name: "isaac mike",
      category: category,
      country: "USA",
    };

    const tx = await program.methods
      .registerDroneOperator(initParams)
      .accounts({
        owner: droneOperator.publicKey,
        droneOperator: droneOperatorAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([droneOperator])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.droneOperator.fetch(
      droneOperatorAccount
    );
    console.log("drone operator account: ", result);
  });

  it("Is register buyer!", async () => {
    // property owner categories
    /* 1: IndividualPropertyOwner,
        2: RealEstateCompany,
        3: CityMunicipality, */

    let category = 1; // IndividualPropertyOwner

    let initParams = {
      name: "mike williams",
      category: category,
      country: "USA",
    };

    const tx = await program.methods
      .registerPropertyOwner(initParams)
      .accounts({
        owner: buyer.publicKey,
        propertyOwner: buyerAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([buyer])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.propertyOwner.fetch(buyerAccount);
    console.log("buyer account: ", result);
  });

  it("Is register seller!", async () => {
    // property owner categories
    /* 1: IndividualPropertyOwner,
        2: RealEstateCompany,
        3: CityMunicipality, */

    let category = 1; // IndividualPropertyOwner

    let initParams = {
      name: "esther johns",
      category: category,
      country: "USA",
    };

    const tx = await program.methods
      .registerPropertyOwner(initParams)
      .accounts({
        owner: seller.publicKey,
        propertyOwner: sellerAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([seller])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.propertyOwner.fetch(sellerAccount);
    console.log("seller account: ", result);
  });

  it("Is claim airspace!", async () => {
    let gpsCoordinates = {
      latitude: "-1.288811",
      longitude: "36.823219",
    };

    let initParams = {
      name: "john doe residence", // name of property
      country: "USA", // country where property is located
      propertyCoordinates: gpsCoordinates, // coordinates of the property
      cubicFeet: 9, // cubic feet volume of the property i.e 3*3*3
    };

    const tx = await program.methods
      .claimAirspace(initParams)
      .accounts({
        owner: propertyOwner.publicKey,
        property: property,
        propertyOwner: propertyOwnerAccount,
        airspace: airspace,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([propertyOwner])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.property.fetch(property);
    console.log("property: ", result);

    let result1 = await program.account.airspace.fetch(airspace);
    console.log("airspace: ", result1);
  });

  it("Is claim airspace by seller!", async () => {
    let gpsCoordinates = {
      latitude: "-1.826252",
      longitude: "37.562728",
    };

    let initParams = {
      name: "wonder residence", // name of property
      country: "USA", // country where property is located
      propertyCoordinates: gpsCoordinates, // coordinates of the property
      cubicFeet: 9, // cubic feet volume of the property i.e 3*3*3
    };

    const tx = await program.methods
      .claimAirspace(initParams)
      .accounts({
        owner: seller.publicKey,
        property: sellerProperty,
        propertyOwner: sellerAccount,
        airspace: airspace,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([seller])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.property.fetch(sellerProperty);
    console.log("property: ", result);

    let result1 = await program.account.airspace.fetch(airspace);
    console.log("airspace: ", result1);
  });

  it("Is rent airspace!", async () => {
    let initParams = {
      cubicFeet: 9, // cubic feet volume of the airspace to rent i.e 3*3*3
    };

    const tx = await program.methods
      .rentAirspace(initParams)
      .accounts({
        owner: droneOperator.publicKey,
        droneOperator: droneOperatorAccount,
        propertyOwner: propertyOwnerAccount,
        property: property,
        configs: configs,
        adminDepositAccount: adminDepositAccount.publicKey,
        adminPdaAuth: adminPdaAuth,
        adminSolVault: adminSolVault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([droneOperator])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.propertyOwner.fetch(
      propertyOwnerAccount
    );
    console.log("property owner account: ", result);

    let result1 = await program.account.property.fetch(property);
    console.log("property: ", result1);

    let result2 = await program.account.configs.fetch(configs);
    console.log("configs: ", result2);
  });

  it("Is buy sell airspace!", async () => {
    let initParams = {
      cubicFeet: 9, // cubic feet volume of the airspace to rent i.e 3*3*3
    };

    const tx = await program.methods
      .buySellAirspace(initParams)
      .accounts({
        owner: buyer.publicKey,
        buyer: buyerAccount,
        seller: sellerAccount,
        property: sellerProperty,
        airspace: airspace,
        configs: configs,
        adminDepositAccount: adminDepositAccount.publicKey,
        adminPdaAuth: adminPdaAuth,
        adminSolVault: adminSolVault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([buyer])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.propertyOwner.fetch(sellerAccount);
    console.log("property owner account: ", result);

    let result1 = await program.account.property.fetch(sellerProperty);
    console.log("property: ", result1);

    let result2 = await program.account.configs.fetch(configs);
    console.log("configs: ", result2);
  });
});
