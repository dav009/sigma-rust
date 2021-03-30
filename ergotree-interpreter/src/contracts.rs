#[cfg(test)]
mod tests {
    use std::convert::TryInto;
    use std::rc::Rc;

    use crate::eval::tests::eval_out_wo_ctx;
    use ergotree_ir::address::AddressEncoder;
    use ergotree_ir::address::NetworkPrefix;
    use ergotree_ir::mir::expr::Expr;
    use ergotree_ir::sigma_protocol::sigma_boolean::SigmaProp;

    #[test]
    fn simplified_age_usd_bank_contract() {
        // simplified version of
        // https://github.com/Emurgo/age-usd/tree/main/ageusd-smart-contracts/v0.4
        /*

          val rcDefaultPrice = 1000000L

          val minStorageRent = 10000000L

          val feePercent = 1

          val HEIGHT = 377771

          val coolingOffHeight: Int = 377770
          val INF = 1000000000L

          val longMax = 9223372036854775807L

          val minReserveRatioPercent = 400L // percent
          val defaultMaxReserveRatioPercent = 800L // percent

            // val dataInput = CONTEXT.dataInputs(0)
            // val validDataInput = dataInput.tokens(0)._1 == oraclePoolNFT
            val validDataInput = true

            // val bankBoxIn = SELF
            // val bankBoxOut = OUTPUTS(0)

            // val rateBox = dataInput
            // val receiptBox = OUTPUTS(1)

            // val rate = rateBox.R4[Long].get / 100
            val rate = 100000000 / 100

            // val scCircIn = bankBoxIn.R4[Long].get
            val scCircIn = 100L
            // val rcCircIn = bankBoxIn.R5[Long].get
            val rcCircIn = 100L
            // val bcReserveIn = bankBoxIn.value
            val bcReserveIn = 100000000L

            // val scTokensIn = bankBoxIn.tokens(0)._2
            val scTokensIn = 100
            // val rcTokensIn = bankBoxIn.tokens(1)._2
            val rcTokensIn = 100

            // val scCircOut = bankBoxOut.R4[Long].get
            val scCircOut = 100
            // val rcCircOut = bankBoxOut.R5[Long].get
            val rcCircOut = 101

            // val scTokensOut = bankBoxOut.tokens(0)._2
            val scTokensOut = 100
            // val rcTokensOut = bankBoxOut.tokens(1)._2
            val rcTokensOut = 99

            val totalScIn = scTokensIn + scCircIn
            val totalScOut = scTokensOut + scCircOut

            val totalRcIn = rcTokensIn + rcCircIn
            val totalRcOut = rcTokensOut + rcCircOut

            val rcExchange = rcTokensIn != rcTokensOut
            val scExchange = scTokensIn != scTokensOut

            val rcExchangeXorScExchange = (rcExchange || scExchange) && !(rcExchange && scExchange)

            // val circDelta = receiptBox.R4[Long].get
            val circDelta = 1L
            // val bcReserveDelta = receiptBox.R5[Long].get
            val bcReserveDelta = 1010000L

            // val bcReserveOut = bankBoxOut.value
            val bcReserveOut = 100000000L + 1010000L

            val rcCircDelta = if (rcExchange) circDelta else 0L
            val scCircDelta = if (rcExchange) 0L else circDelta

            val validDeltas = (scCircIn + scCircDelta == scCircOut) &&
                              (rcCircIn + rcCircDelta == rcCircOut) &&
                              (bcReserveIn + bcReserveDelta == bcReserveOut)

            val coinsConserved = totalRcIn == totalRcOut && totalScIn == totalScOut

            // val tokenIdsConserved = bankBoxOut.tokens(0)._1 == bankBoxIn.tokens(0)._1 && // also ensures that at least one token exists
            //                         bankBoxOut.tokens(1)._1 == bankBoxIn.tokens(1)._1 && // also ensures that at least one token exists
            //                         bankBoxOut.tokens(2)._1 == bankBoxIn.tokens(2)._1    // also ensures that at least one token exists

            val tokenIdsConserved = true

            // val mandatoryRateConditions = rateBox.tokens(0)._1 == oraclePoolNFT
            val mandatoryRateConditions = true
            val mandatoryBankConditions = //bankBoxOut.value >= $minStorageRent &&
                                          rcExchangeXorScExchange &&
                                          coinsConserved &&
                                          validDeltas &&
                                          tokenIdsConserved

            // exchange equations
            val bcReserveNeededOut = scCircOut * rate
            val bcReserveNeededIn = scCircIn * rate
            val liabilitiesIn = max(min(bcReserveIn, bcReserveNeededIn), 0)


            val maxReserveRatioPercent = if (HEIGHT > coolingOffHeight) defaultMaxReserveRatioPercent else INF

            val reserveRatioPercentOut =
                if (bcReserveNeededOut == 0) maxReserveRatioPercent else bcReserveOut * 100 / bcReserveNeededOut

            val validReserveRatio = if (scExchange) {
              if (scCircDelta > 0) {
                reserveRatioPercentOut >= minReserveRatioPercent
              } else true
            } else {
              if (rcCircDelta > 0) {
                reserveRatioPercentOut <= maxReserveRatioPercent
              } else {
                reserveRatioPercentOut >= minReserveRatioPercent
              }
            }

            val brDeltaExpected = if (scExchange) { // sc
              val liableRate = if (scCircIn == 0) longMax else liabilitiesIn / scCircIn
              val scNominalPrice = min(rate, liableRate)
              scNominalPrice * scCircDelta
            } else { // rc
              val equityIn = bcReserveIn - liabilitiesIn
              val equityRate = if (rcCircIn == 0) rcDefaultPrice else equityIn / rcCircIn
              val rcNominalPrice = if (equityIn == 0) rcDefaultPrice else equityRate
              rcNominalPrice * rcCircDelta
            }

            val fee = brDeltaExpected * feePercent / 100

            val actualFee = if (fee < 0) {fee * -1} else fee
            // actualFee is always positive, irrespective of brDeltaExpected

            val brDeltaExpectedWithFee = brDeltaExpected + actualFee

            mandatoryRateConditions &&
            mandatoryBankConditions &&
            bcReserveDelta == brDeltaExpectedWithFee &&
            validReserveRatio &&
            validDataInput
        }
                 */
        let p2s_addr_str = "7Nq5tKsVYCgneNgEfA2BJKwGsWozezNLhCNsRBihcHVFkDTuTThd4Qt1bi7NfCK1HuuVfjksMrEftV6MEFajjuyp1TMD2PX7SYWvkg9zH4CtgpdoBjekCNXs5XawxXnW6FT7GCqXTpJUP2TkkuqBh1df99PTigehys36uZz9wQnkrJXrv3mw3Yy4CM622qe5wdqLtpEonjazEmsw8weqEYegDyfJnswDvDkLPXtcCB86i19jik4fnSTtCcYj3jpWCQ7WL5dZn1ivs5JGRsR2ioNCRiZd3Gu1zJBgbHkMg41Z6VeCRWXjGY99BUtgtQiepSHGHajFCVcFAHhVxccdVUPCxGeEL6c2dNx6qzEkVfTfHs5qBgJewR8KCZTCVTurNBHeqCSVdxnfFvhW3f72cNrae5E1UhTAXU2iX4LZMHQsKyefY24Aq1b1srTyRWLpixjbcezFqA2TKjGSn1p1ruxbR7AQpW24ByPKT9sFE9ii4qNeXDnLcGtAAGS9FC5SD1s516a4NCu6v9zZfTvRKGkCwt78J8DEVnhTbttjcsvqFsUXQrvAv7TGVsaT4mL6B7F5BhRoZwFkgRXqFUVCWvgqJrwwjFRtbc5aZz";
        let encoder = AddressEncoder::new(NetworkPrefix::Mainnet);
        let addr = encoder.parse_address_from_str(p2s_addr_str).unwrap();
        let script: Rc<Expr> = addr.script().unwrap().proposition().unwrap();
        dbg!(&script);
        let res: bool = eval_out_wo_ctx::<SigmaProp>(script.as_ref())
            .try_into()
            .unwrap();
        assert!(res);
    }

    #[test]
    fn amm_simple_pool() {
        // from eip-14 https://github.com/ergoplatform/eips/pull/27/files
        let p2s_addr_str = "k6fD5ht5e1itDejPFV2VzAoHv478KQCbDnLAL6XUVeEu8KDaboCVZAoFz2AtMoLqM3CgQfr2TZhpwz7K96AgwTXDvBVeTchJ31jjD46Di1W67H8wwFcivnY62UB6L7HWzCkbYuiZaAq2qSJta5Twt4A2Aaoy7xViWcyLUVNAyQYDJXKhVBAGwp76i2too5yWUmEU4zt9XnjJAUt1FFfurNtTNHNPDbqmTRE4crz347q6rfbvkMmg9Jtk9rSiPCQpKjdbZVzUnP4CUw6AvQH6rZXxgNMktAtjQdHhCnrCmf78FwCKqYS54asKd1MFgYNT4NzPwmdZF6JtQt1vvkjZXqpGkjy33xxDNYy8JZS8eeqVgZErPeJ1aj4aaK8gvmApUgGStMDFeFYjuQqZiZxEAHNdAXDg7hyGnmfzA6Hj9zcB7p9nKCDNhEQEMPL1kMG5aXvt2HUPXqiCkLrv596DaGmRMN3gMJaj1T1AfMYNwZozcJ9uUSK4i6Xham28HWAekTtDPhobnmjvkubwLVTtvUumWHtDWFxYSJPF7vqzgZqg6Y5unMF";
        let encoder = AddressEncoder::new(NetworkPrefix::Mainnet);
        let addr = encoder.parse_address_from_str(p2s_addr_str).unwrap();
        assert!(addr.script().unwrap().proposition().is_ok());
        let script: Rc<Expr> = addr.script().unwrap().proposition().unwrap();
        dbg!(&script);
        // let res: bool = eval_out_wo_ctx::<SigmaProp>(script.as_ref())
        //     .try_into()
        //     .unwrap();
        // assert!(!res);
    }

    #[test]
    fn amm_simple_swap() {
        // from eip-14 https://github.com/ergoplatform/eips/pull/27/files
        let p2s_addr_str = "cLPHJ3MHuKAHoCUwGhcEFw5sWJqvPwFyKxTRj1aUoMwgAz78Fg3zLXRhBup9Te1WLau1gZXNmXvUmeXGCd7QLeqB7ArrT3v5cg26piEtqymM6j2SkgYVCobgoAGKeTf6nMLxv1uVrLdjt1GnPxG1MuWj7Es7Dfumotbx9YEaxwqtTUC5SKsJc9LCpAmNWRAQbU6tVVEvmfwWivrGoZ3L5C4DMisxN3U";
        let encoder = AddressEncoder::new(NetworkPrefix::Mainnet);
        let addr = encoder.parse_address_from_str(p2s_addr_str).unwrap();
        let script: Rc<Expr> = addr.script().unwrap().proposition().unwrap();
        dbg!(&script);
        // let res: bool = eval_out_wo_ctx::<SigmaProp>(script.as_ref())
        //     .try_into()
        //     .unwrap();
        // assert!(!res);
    }

    #[test]
    fn amm_conc_pool_root() {
        // from eip-14 https://github.com/ergoplatform/eips/pull/27/files
        let p2s_addr_str = "3STRfQWC9Xb5wAxBiEQ74uTFSemk1oHn43mwj9tMCeu2a3A4kie1bY2qsCdRaEmdQoq3B4tXQuzq9nm84A8PmBgCzgGDEZf2pgYoAUc6krZxUY3rvKWW44ZpzN3u5bFRpKDo6rxKtxX2tw99xmfyfaVBejgDaTfsib2PSVsu9hrLQ3SouECWHQMjDA3Pi8ZuCvQeW8GDkZfHPr3SgwaxY1jpY2njsmf3JBASMoVZ6Mfpg63Q6mBno7mKUSCE7vNHHUZe2V7JEikwjPkaxSWxnwy3J17faGtiEHZLKiNQ9WNtsJLbdVp56dQGfC2zaiXjhx1XJK6m4Nh2M8yEvSuBzanRBAJqrNseGS97tk2iLqqfHrqqmmDsHY3mujCURky4SLr7YLk4B";
        let encoder = AddressEncoder::new(NetworkPrefix::Mainnet);
        let addr = encoder.parse_address_from_str(p2s_addr_str).unwrap();
        let script: Rc<Expr> = addr.script().unwrap().proposition().unwrap();
        dbg!(&script);
        // let res: bool = eval_out_wo_ctx::<SigmaProp>(script.as_ref())
        //     .try_into()
        //     .unwrap();
        // assert!(!res);
    }
}
